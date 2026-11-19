# Design: generated planning code (levels B1 and B2)

Status: implemented. Covers #123 to #131 under the tracking issue #132; Level A
(#122) is in [`pddl-macros.md`](pddl-macros.md).

## 1. Crates

```text
pddl-runtime   Task trait, BitSet/BitRelation/BitTable/Values, searches (feature `search`); no dependencies
pddl-codegen   AST -> TokenStream for B1 and B2; depends on pddl-core (parser, sas-plus), quote, proc-macro2
pddl-macros    #[task(...)] attribute (feature `codegen`) on top of pddl-codegen
pddl           re-exports pddl-runtime as `pddl::runtime` and the attribute as `pddl::task`, feature `codegen`
pddl-bench     unpublished: runtime grounder baseline, fixtures, cross-checks, criterion benchmarks
```

Generated code names `::pddl::runtime::...` and `::pddl::...`, so a user depends on
`pddl` alone. `pddl-runtime` has no `pddl` dependency, which `cargo tree -p pddl-runtime`
shows.

## 2. The runtime baseline (#124)

**Question.** Generated code has to beat a realistic runtime grounder. #124 asked
whether to use miniplan's grounder (`pddl` -> `miniplan` -> `pddl` is a cycle) or
move a grounder into `pddl-runtime`.

**Decision: neither. The baseline is `pddl::sas_plus::ground` (#168), run by
`pddl_bench::GroundTask`.** Since #124 was written, `pddl-core` gained a grounder
that does what Fast Downward's translator does: it instantiates actions while it
computes relaxed reachability (#169), evaluates static predicates away, and writes
a SAS+ task with the same operators as Fast Downward on every fixture. That is the
"Fast-Downward-style grounder" #124 asks for, and it lives in this repository, so:

- there is no dependency on miniplan and no cycle, on crates.io or in the workspace;
- miniplan's release cycle is unaffected; miniplan can switch to `pddl::sas_plus`
  if it wants to share the grounder;
- the open question about miniplan's state layout is moot.

`GroundTask` packs every (binary) variable into one bit and every operator into
masks: precondition-true, precondition-false, add and delete, plus conditional
effects whose conditions are read from the state before the action. Successors
are one pass over all operators. That is a fair baseline: states are `Box<[u64]>`,
not a `Vec` of values, and applicability is a few word operations.

It sits in the unpublished `pddl-bench` crate, not in `pddl-runtime`, because it
needs the grounder and `pddl-runtime` must not depend on `pddl`. If users want a
runtime fallback for domains unknown at compile time, it can move to
`pddl::sas_plus` behind the `sas-plus` feature without changes.

What the baseline does not do: Fast Downward's mutex groups (multi-valued
variables) and successor generator (a decision tree). Both would make it faster;
neither changes the comparison much on the fixtures here.

**Oracle.** `pddl-bench/tests/baseline.rs` checks that A* (with `h = 0`) and IDA*
on `GroundTask` find the optima Fast Downward and an exhaustive search
(`sas_plus::Task::perfect_heuristic`) give, and that every plan replays.

## 3. Level B1: a lifted domain, problems at run time (#126)

```rust
#[pddl::task(domain = "blocks.pddl")] // or domain_source = r#"(define ...)"#
pub mod blocks {}

let task = blocks::Task::new(&problem)?;            // problem loaded at run time
let plan = pddl::runtime::search::astar(&task, |s| task.unsatisfied_goals(s));
```

The attribute goes on a module because B1 emits items, not an expression; items
the user writes into the module stay. `build.rs` users call
`pddl_codegen::lifted(&domain)` and `include!` the tokens from `OUT_DIR`
(`pddl-bench/build.rs` does that for gripper).

What the module holds:

| Item | Content |
|---|---|
| `types::Block(u32)` | one id per PDDL type, implementing `pddl_runtime::Id` |
| `SIGNATURE` | types, predicates and constants for `pddl::codegen::load` |
| `State` | one field per fluent predicate: `bool`, `BitSet`, `BitRelation`, `BitTable` |
| `Statics` | the same for predicates no action changes, kept outside the state |
| `Action` | `Copy` enum, one variant per action with typed fields |
| `Task` | `new(&Problem)`, `describe(action)`, `fact(..)`, and `impl pddl_runtime::Task` |

**Numbering.** The loader numbers objects type by type in depth-first order of
the type tree, so a type and its subtypes are one range `starts[t]..ends[t]`.
Inside generated code every bound variable is a global object number; a
predicate argument of type `A` is stored as `number - starts[A]`. Where a
variable's type is not a subtype of `A`, a range check guards the access. An
object with several types (an `either` type, or a constant redeclared with
another type) is a `LoadError`, because it would break the ranges.

**Join order.** Successor generation is nested loops over the parameters in
declaration order. Each conjunct of the precondition is checked right after
its last variable is bound, at compile time, so a failing static or fluent
literal prunes the loops below it.

**Effects.** `apply` clones the state, applies every delete, then every add,
so an add wins over a delete of the same fact, as PDDL requires.

**Goals** are only known when the problem loads, so `pddl::codegen::load`
grounds them into a `pddl_runtime::Formula` over predicate indices, which
`Task::fact` evaluates. `unsatisfied_goals` counts the top-level conjuncts that
don't hold.

**Identifiers.** Types and actions become `CamelCase`, predicates and
parameters `snake_case`; Rust keywords get a trailing `_` (`?type` is
`type_`). Names that map to the same identifier (`on-table`, `on_table`) are a
compile error that names them.

**Supported** in B1 itself: STRIPS, `:typing`, `:negative-preconditions`,
`:equality`. Other requirements are compile errors
(`unsupported requirement: :durative-actions`), as are durative actions,
preferences and constraints.

**Checks.** `pddl-bench/tests/lifted.rs` walks the generated task and
`GroundTask` in lockstep and compares the applicable actions in every state:
all states of gripper-2 and gripper-4, and 3000 states each of bw-large-a and
logistics-e. Actions that lead back to the same state are left out on both
sides, because the grounder drops them, as Fast Downward does. Breadth-first
plans have the known optimal costs and replay.
