# Changelog

All notable changes to `pddl-macros` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `domain!`, `problem!` and `pddl!` check PDDL at compile time and expand to a
  `LazyLock` of `pddl::Domain`, `pddl::Problem` or `pddl::PddlFile`.
  `validated!` expands to the checked source text. Each has a `_file` form
  that reads a file relative to the calling file and rebuilds when it changes.
- Every syntax error becomes a compile error with its line and column, the
  parser's help, and the offending line.
