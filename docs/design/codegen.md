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
