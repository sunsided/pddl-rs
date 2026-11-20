# Changelog

All notable changes to `pddl-runtime` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `Task`, the planning-task trait that planners are generic over: initial
  state, successors, goal test and action costs, with static dispatch.
- Fact storage for generated states: typed ids (`Id`), `BitSet` for unary
  predicates, `BitRelation` for binary ones, `BitTable` for any arity and
  `Values` for numeric fluents. All are cheap to clone and hash.
