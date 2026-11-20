# Changelog

All notable changes to `pddl-validate` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `Validator`, an independent validator for sequential classical plans on the
  `pddl` syntax tree: STRIPS, typing, negative and disjunctive preconditions,
  equality, `exists`, `forall`, `imply`, conditional effects and stratified
  derived predicates. It checks a domain and problem once (`ModelError` for
  undeclared names, wrong arities, unbound variables and a variable declared
  twice in one list) and then runs any
  number of plans.
- `Report`, with a `ValidationStatus` of `Valid`, `Invalid` or `Unsupported`.
  An invalid plan has a `Violation` naming the step, the failing condition as
  specific as it allows (`Unmet`) and a VAL-style repair hint (`advice`).
  Constructs outside the supported scope make the report `Unsupported`, never
  valid. The report traces the atoms each step adds and deletes.
- Numeric fluents: comparisons in conditions and goals, `assign`, `increase`,
  `decrease`, `scale-up` and `scale-down`, action costs and the problem's
  `:metric` (`Report::metric`). Comparisons allow a relative tolerance
  (`Validator::with_tolerance`, 1e-12 by default, so integers below 1e12
  compare exactly). Effects that read undefined fluents, divide by zero or
  overflow, and conflicting updates of one fluent, make a plan invalid.
  Conditions follow three-valued (Kleene) logic: an undefined value, a
  division by zero or an overflow makes only its part unknown, so
  `(or (p) (> (f) 0))` holds when `(p)` does, while a precondition or goal
  that stays unknown is invalid. A derived-predicate rule whose body is
  unknown derives nothing, and an undefined metric leaves the plan's verdict
  as it is, with no `Report::metric`.
- Temporal plans: durative actions with `at start`, `over all` and `at end`
  conditions and effects, duration constraints and inequalities, conditional
  effects on `at start` or `at end` conditions, linear continuous effects,
  timed initial literals, PDDL 2.1 interference with an epsilon separation
  (`Validator::with_epsilon`, 0.01 by default) and `total-time` as the
  makespan. Interfering events exactly epsilon apart pass at any plan time.
  Timed initial literals that set one atom both true and false at one time
  are a `ModelError` (`ContradictoryTimedLiterals`). Durations meet their
  constraints within an absolute tolerance
  (`Validator::with_duration_tolerance`, 0.0005 by default, so durations
  rounded to three decimals pass), and a ground durative action overlapping
  itself is `SelfOverlap`. `Plan` reads `t: (action args) [d]` steps. Nonlinear dynamics and
  derived predicates in temporal plans are `Unsupported`.
- PDDL+: processes and events on the temporal timeline. Processes add linear
  rates while their precondition holds; events fire when theirs holds, in
  cascades (`EventCascade` after 1000 rounds at one time). Switching times are
  the exact roots of the linear conditions. Nonlinear dynamics, discrete
  process effects and Zeno behaviour are `Unsupported`, and so are dynamics
  that need more than 100 000 segments between two plan happenings.
- `Plan`, which reads plan files as planners write them: one `(action args)`
  per step, `;` comments and cost lines, CRLF line ends and a byte order mark.
