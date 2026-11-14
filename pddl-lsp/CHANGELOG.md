# Changelog

All notable changes to `pddl-lsp` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Arguments whose declared type doesn't fit the parameter are reported, such as
  `(at t1 t1)` where the second parameter of `at` is a `place`. Subtypes and
  `(either ...)` types fit; untyped names and types the workspace doesn't know
  are not checked.
- Constructs the `:requirements` don't allow get a warning, such as `forall` in a
  precondition without `:universal-preconditions`, `when` without
  `:conditional-effects`, or typed names without `:typing`. Shorthands such as
  `:adl` and `:fluents` are expanded, and a problem also has its domain's
  requirements.

## [0.1.0] - 2026-11-14

[0.1.0]: https://github.com/sunsided/pddl-rs/releases/tag/pddl-lsp-v0.1.0

### Added

- A language server for PDDL over stdio: diagnostics for syntax errors, unknown names, wrong
  argument counts, undeclared variables and duplicate definitions, checked across domain and
  problem files; hover, go to definition, references, rename, document and workspace symbols,
  completion, signature help, formatting, quick fixes, folding and semantic highlighting.
- Inlay hints with parameter names before the arguments of predicates and functions.
- Hints for unused types, constants, predicates, functions, objects and parameters.
