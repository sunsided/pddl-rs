# Changelog

All notable changes to `pddl-codegen` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- `lifted` turns a PDDL domain into typed Rust planning code (level B1): an
  id type per PDDL type, a `State` with one field per predicate, the static
  predicates in a separate table, an `Action` enum, successor generation that
  checks each precondition as soon as its variables are bound, and a `Task`
  that loads problems at run time and implements `pddl_runtime::Task`.
