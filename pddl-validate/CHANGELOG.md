# Changelog

All notable changes to `pddl-validate` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `Validator`, an independent validator for sequential classical plans on the
  `pddl` syntax tree: STRIPS, typing, negative and disjunctive preconditions,
  equality, `exists`, `forall`, `imply`, conditional effects and stratified
  derived predicates. It checks a domain and problem once (`ModelError` for
  undeclared names, wrong arities and unbound variables) and then runs any
  number of plans.
- `Report`, with a `ValidationStatus` of `Valid`, `Invalid` or `Unsupported`.
  An invalid plan has a `Violation` naming the step, the failing condition as
  specific as it allows (`Unmet`) and a VAL-style repair hint (`advice`).
  Constructs outside the supported scope make the report `Unsupported`, never
  valid. The report traces the atoms each step adds and deletes.
- Numeric fluents: comparisons in conditions and goals, `assign`, `increase`,
  `decrease`, `scale-up` and `scale-down`, action costs and the problem's
  `:metric` (`Report::metric`). Comparisons allow a relative tolerance
  (`Validator::with_tolerance`). Undefined fluents, division by zero,
  overflow and conflicting updates of one fluent make a plan invalid.
- Temporal plans: durative actions with `at start`, `over all` and `at end`
  conditions and effects, duration constraints and inequalities, conditional
  effects on `at start` or `at end` conditions, linear continuous effects,
  timed initial literals, PDDL 2.1 interference with an epsilon separation
  (`Validator::with_epsilon`, 0.01 by default) and `total-time` as the
  makespan. `Plan` reads `t: (action args) [d]` steps. Nonlinear dynamics and
  derived predicates in temporal plans are `Unsupported`.
- PDDL+: processes and events on the temporal timeline. Processes add linear
  rates while their precondition holds; events fire when theirs holds, in
  cascades (`EventCascade` after 1000 rounds at one time). Switching times are
  the exact roots of the linear conditions. Nonlinear dynamics, discrete
  process effects and Zeno behaviour are `Unsupported`.
- `Plan`, which reads plan files as planners write them: one `(action args)`
  per step, `;` comments and cost lines, CRLF line ends and a byte order mark.
