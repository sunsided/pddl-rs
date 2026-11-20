# Changelog

All notable changes to `pddl-codegen` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Licensed under `EUPL-1.2 OR MPL-2.0`, at your option.

### Added

- `lifted` turns a PDDL domain into typed Rust planning code (level B1): an
  id type per PDDL type, a `State` with one field per predicate, the static
  predicates in a separate table, an `Action` enum, successor generation that
  checks each precondition as soon as its variables are bound, and a `Task`
  that loads problems at run time and implements `pddl_runtime::Task`.
- `:adl` in generated code: `exists` and `forall` become loops over the
  bound variable's objects, `or`, `imply` and `not` plain boolean code, and
  `forall` and `when` effects loops and branches whose conditions read the
  state before the action.
- Numeric fluents: functions are `Values` fields of `State` (or `Statics`),
  comparisons and `assign`, `increase`, `decrease`, `scale-up`, `scale-down`
  are generated arithmetic, and `(increase (total-cost) ...)` over static
  functions becomes `Task::cost` when the problem minimizes `total-cost`.
- Derived predicates: stratified when the crate compiles (a predicate that
  depends on its own negation is a compile error), kept in `State` and
  recomputed from the other facts after every action and for the initial
  state.
- `grounded` grounds a domain and problem with `pddl::sas_plus::ground` and
  generates the task as constant mask tables over a `[u64; WORDS]` state
  (level B2), with `Limits` on its size.
- `lifted_with` and `Bindings`: PDDL types bound to Rust types, whose values
  live in a side arena `Bound` next to the state, and static predicates and
  functions attached to pure Rust functions.
