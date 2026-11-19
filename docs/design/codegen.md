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

## 4. `:adl` in B1 (#128)

- `exists` and `forall`, in preconditions and goals, become `any` and `all`
  over the bound variable's object range; a variable of an `either` type
  checks membership first. Quantified variables get slots of their own after
  the parameters, so the join order only looks at the parameters a conjunct
  reads: `(exists (?s) (and (in ?s ?r) (on ?s)))` is checked as soon as `?r` is
  bound.
- `or`, `imply` (as `(or (not a) b)`) and `not` over any formula are plain
  boolean code.
- `forall` effects are loops and `when` effects branches. Their conditions read
  `state`, the state before the action, and change `next`; deletes of all
  effects run before adds.

Checks (`pddl-bench/tests/adl.rs`): lockstep against the grounder on
sussman-anomaly and get-paid4 (`when`, `forall` effects), and a `lights`
domain with `exists`, `forall`, `or` and `imply`, which the grounder rejects,
against hand-computed optima.

## 5. Numeric fluents in B1 (#129)

- Every function except `total-cost` is a `pddl_runtime::Values` field: in
  `State` if an effect changes it, in `Statics` otherwise. `Values` hashes
  canonical bits (`-0.0` is `0.0`; undefined is one NaN), so states with
  fluents still deduplicate.
- **Undefined values** follow PDDL 2.1. A comparison that reads an undefined
  value makes the whole precondition or `when` condition fail, also under
  `not` (a `Cell` flag, set by the comparison, checked after each conjunct).
  An action whose numeric effect reads an undefined value is not applicable:
  `apply_<n>` returns `None`, and successors skip it.
- `assign` evaluates its value in the state before the action; `increase`,
  `decrease`, `scale-up` and `scale-down` apply to the state being built, so
  several increases of one fluent add up. A `scale-down` by 0 is undefined.
- **Costs.** `(increase (total-cost) e)` is not a state fluent: it becomes
  `Task::cost`, used when the problem has `(:metric minimize (total-cost))`
  and 1 otherwise. `e` must be a sum or product of non-negative integer
  literals and static functions, at the top level of the effect; the loader
  rejects initial values of those functions that are not non-negative
  integers (`LoadError::InvalidCost`), and an undefined cost makes the action
  inapplicable. Actions without a cost effect cost 0 under the metric, as in
  the grounder.
- The parser reads `(= (f) (g))` as an equality of terms and
  `(assign (f) (g))` as an object assignment; for numeric functions the model
  turns both into their numeric meaning.

Checks (`pddl-bench/tests/numeric.rs`): lockstep and per-operator costs
against the grounder on gripper-costs and courier, optima 7 and 16; a `fuel`
domain with numeric preconditions and effects against a hand-computed optimum;
undefined values; invalid costs. No reference planner for general numeric
domains was run (none installed): the grounder rejects numeric fluents other
than costs.
