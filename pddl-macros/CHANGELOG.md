# Changelog

All notable changes to `pddl-macros` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Licensed under `EUPL-1.2 OR MPL-2.0`, at your option, instead of `EUPL-1.2`
  alone. Versions already published keep the licence they were published under.

### Added

- With the `codegen` feature, `#[task(domain = "...")]` fills a module with
  typed planning code that `pddl-codegen` generates from the domain. With
  `problem = "..."` it grounds the task while the crate compiles, within
  `max_operators` and `max_facts`. `types(...)` binds PDDL types to Rust types
  and `attach(...)` backs predicates and functions with Rust functions.
- `domain!`, `problem!` and `pddl!` check PDDL at compile time and expand to a
  `LazyLock` of `pddl::Domain`, `pddl::Problem` or `pddl::PddlFile`.
  `validated!` expands to the checked source text. Each has a `_file` form
  that reads a file relative to the calling file and rebuilds when it changes.
- Every syntax error becomes a compile error with its line and column, the
  parser's help, and the offending line.
