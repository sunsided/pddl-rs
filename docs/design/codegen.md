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
compile error that names them. The generated locals that could meet a PDDL
name start with `__`: matches bind action fields as `__p0`, `__p1`, ..., and
`Task::new` takes the closure of a bound type `robot` as `__robot`.

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
  So that the result does not depend on the order of the parts, `and`, `or`
  and quantifiers over a comparison evaluate every part (`&`, `|`, `fold`),
  and so does the goal's `Formula`. A non-finite result of arithmetic, as from
  an overflow, is undefined too, as is a non-finite literal or attachment value.
  An action whose numeric effect reads an undefined value or yields a
  non-finite one is not applicable: `apply_<n>` returns `None`, and successors
  skip it.
- `assign` evaluates its value in the state before the action; `increase`,
  `decrease`, `scale-up` and `scale-down` apply to the state being built, so
  several increases of one fluent add up. A `scale-down` by 0 is undefined.
- **Costs.** `(increase (total-cost) e)` is not a state fluent: it becomes
  `Task::cost`, used when the problem has `(:metric minimize (total-cost))`
  and 1 otherwise. `e` must be a sum or product of non-negative integer
  literals and static functions, at the top level of the effect; under the
  metric the loader rejects initial values of those functions that are not
  non-negative integers (`LoadError::InvalidCost`), and an undefined cost
  makes the action inapplicable. Actions without a cost effect cost 0 under the metric, as in
  the grounder.
- The parser reads `(= (f) (g))` as an equality of terms and
  `(assign (f) (g))` as an object assignment; for numeric functions the model
  turns both into their numeric meaning, and the loader does the same for a
  goal `(= (f) (g))`.

Checks (`pddl-bench/tests/numeric.rs`): lockstep and per-operator costs
against the grounder on gripper-costs and courier, optima 7 and 16; a `fuel`
domain with numeric preconditions and effects against a hand-computed optimum;
undefined values; invalid costs. No reference planner for general numeric
domains was run (none installed): the grounder rejects numeric fluents other
than costs.

## 6. Derived predicates in B1 (#130)

- A `:derived` rule becomes an `ActionInfo` named after its predicate: its
  parameters are the head's arguments, its precondition the body. Several
  rules for one predicate are a disjunction.
- **Strata, at compile time.** A rule's stratum is at least the stratum of
  every derived predicate its body reads, and one more than every one it reads
  under an odd number of `not`s. A predicate that would need a stratum beyond
  the number of derived predicates depends on its own negation: a compile
  error (`tests/ui/task_not_stratifiable.rs`).
- **Evaluation.** Derived facts live in `State`, so goals and preconditions
  read them like other facts. `derive` clears them, then fires each stratum's
  rules, lowest first, until nothing changes. It runs at the end of every
  `apply` and on the initial state, so derived facts are a function of the
  basic facts and never make two states with the same basic facts differ.
  Recomputing from scratch is the simple choice; computing them lazily was
  not measured.
- An action that changes a derived predicate is a compile error.

Checks: `pddl-bench/tests/derived.rs` solves `wiring` (a recursive derived
predicate) at Fast Downward's optimum of 4 with A* and IDA*, and checks the
derived facts after single steps; `model.rs` tests the stratum order.

## 7. Level B2: domain and problem fixed (#127)

```rust
#[pddl::task(domain = "blocks.pddl", problem = "p1.pddl", max_operators = 20_000)]
mod p1 {}
let plan = pddl::runtime::search::breadth_first(&p1::Task);
```

- The attribute grounds the task while the crate compiles, with the same
  `sas_plus::ground` the runtime baseline uses, so B2 and the baseline have the
  same operators and facts.
- Every changing ground atom is one bit; the state is `[u64; WORDS]` with
  `WORDS` a constant, so it is `Copy`-sized and hashes without a heap pointer.
- Each operator is one row of `static OPERATORS: [Operator; M]`: four masks and
  a cost. The successor loop is the same code for every operator, as #127
  asks: tables, not code per action, so rustc only sees data.
- STRIPS with negative preconditions only: conditional effects are a compile
  error (masks can't express them), and so are numeric fluents (the grounder
  rejects them).
- **Size.** Compile time grows linearly with the operators: each adds
  `4 * WORDS` words of constant data. `max_operators` (default 100 000) and
  `max_facts` (default 10 000) fail with a message that names the count and
  the limit (`tests/ui/task_too_large.rs`).

Checks (`pddl-bench/tests/grounded.rs`): lockstep against `GroundTask` on all
of gripper-4 and 3000 states of bw-large-a, the gripper-4 optimum, and replay.

## 8. Reference searches and benchmarks (#125)

- `pddl_runtime::search` (feature `search`): `breadth_first`, `astar(task, h)`
  and `ida_star(task, h)`, generic over `Task`, each returning the plan and the
  number of expanded states; `replay` checks a plan against `successors` and
  `apply`. Reference searches, not a planner.
- `pddl-bench/tests/cross_check.rs`: on gripper-2, gripper-4 and a small
  logistics problem (`pddl-bench/fixtures/logistics-small.pddl`), the runtime
  grounder, B1 and B2 each find the optimal cost with all three searches, and
  every plan replays. Costs are compared, not lengths, and only for optimal
  searches; A* with the goal count is not admissible and only gets a replay
  check.
- `cargo bench -p pddl-bench --bench compare` prints the table below;
  `--bench search` is the criterion version for gripper-4.

Numbers from one run on the development machine (release build, Rust 1.100):

| instance | search | implementation | expanded | time to solve | states/s |
|---|---|---|---:|---:|---:|
| gripper-4 | breadth-first | runtime grounder | 218 | 0.066 ms | 3282490 |
| gripper-4 | breadth-first | B1 (generated) | 218 | 0.257 ms | 847240 |
| gripper-4 | breadth-first | B2 (grounded at compile time) | 218 | 0.025 ms | 8806302 |
| bw-large-a | A*, goal count | runtime grounder | 9 | 0.024 ms | 373506 |
| bw-large-a | A*, goal count | B1 (generated) | 9 | 0.026 ms | 347195 |
| bw-large-a | A*, goal count | B2 (grounded at compile time) | 9 | 0.013 ms | 717246 |
| logistics-small | breadth-first | runtime grounder | 46 | 0.018 ms | 2623325 |
| logistics-small | breadth-first | B1 (generated) | 46 | 0.071 ms | 648125 |
| logistics-small | breadth-first | B2 (grounded at compile time) | 46 | 0.004 ms | 10548039 |

Reading them:

- **B2 is the fastest**, 2–4× the runtime grounder on every instance: a `[u64; N]` state needs
  no allocation to clone or hash, and the mask loop has a constant length.
- **B1 is slower than the runtime grounder**, about 4× on gripper-4 and on
  logistics-small. Its
  state has one heap-allocated bit set per predicate, so every successor clones
  several boxes where the grounder clones one, and its loops enumerate every
  object of a parameter's type where the grounder only scans operators that
  survived grounding. B1's gain is elsewhere: it loads any problem of the
  domain at run time without grounding, in memory that grows with the objects,
  not the ground actions. Packing all predicates into one bit vector per state
  is the obvious next step; it is not done here.
- bw-large-a with the goal count solves in 9 expansions, so its row measures
  little beyond setup. A heavier instance (logistics-e with A* and the goal
  count) did not finish in five minutes with any of these reference searches.

## 9. Bound types and semantic attachments (#131)

```rust
#[pddl::task(
    domain = "rovers.pddl",
    types(place = crate::Cell),
    attach(adjacent = crate::adjacent, distance = crate::distance),
)]
mod rovers {}

let task = rovers::Task::new(&problem, |name| Cell::parse(name))?;
fn adjacent(bound: &rovers::Bound, a: rovers::types::Place, b: rovers::types::Place) -> bool { .. }
```

The two open questions of #131, decided:

- **Syntax: attribute arguments**, not a side file next to the `.pddl`. The
  bindings name Rust paths, which belong in Rust source, and the attribute
  already carries the domain. `-` in a PDDL name is written `_`.
- **Attachments see the bound values, not the state.** An attachment gets
  `&Bound` and its arguments as typed ids. So an attached predicate or
  function must be static: one that an action or rule changes is a compile
  error. This keeps attachments pure for a given binding, which search relies
  on, and keeps `State` the only thing that changes.

How it works:

- A bound type gets a field in `Bound`: `Vec<crate::Cell>`, one value per object
  of the type and its subtypes, in id order. `Task::new` takes one
  `FnMut(&str) -> T` per bound type and calls it with each object's name. The
  state keeps ids only, as #131 asks, so cloning and hashing stay cheap.
- An attached predicate has no storage: preconditions, goals and rules call the
  function where they would read the fact, and the problem's initial facts for
  it are ignored. An attached function returns `f64` and is read where a
  fluent would be, including in action costs.
- Bindings need the lifted form; with a `problem` they are a compile error.

Checks: `pddl-bench/tests/bindings.rs` plans on a grid whose adjacency and
distances come from attachments over bound cell coordinates; `task.rs` tests
the errors.
