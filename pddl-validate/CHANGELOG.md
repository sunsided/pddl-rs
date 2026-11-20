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
- `Plan`, which reads plan files as planners write them: one `(action args)`
  per step, `;` comments and cost lines, CRLF line ends and a byte order mark.
