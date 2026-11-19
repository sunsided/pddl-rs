# pddl-runtime

The planning-task trait and the fact storage that code generated from PDDL
(by `pddl-codegen`, through the `pddl` crate's `codegen` feature) builds on.
It does not depend on the PDDL parser.

- `Task`: initial state, successors, goal test and action costs. Planners are
  generic over it and monomorphize per domain.
- `Id`, `BitSet`, `BitRelation`, `BitTable`: facts of predicates of any arity,
  over typed object ids.
- `Values`: numeric fluents, hashable.
- With the `search` feature: blind breadth-first search, A* and IDA*, and plan
  replay. These are reference searches for tests and benchmarks.
