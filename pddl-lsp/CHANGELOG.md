# Changelog

All notable changes to `pddl-lsp` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- A language server for PDDL over stdio: diagnostics for syntax errors, unknown names, wrong
  argument counts, undeclared variables and duplicate definitions, checked across domain and
  problem files; hover, go to definition, references, rename, document and workspace symbols,
  completion, signature help, formatting, quick fixes, folding and semantic highlighting.
- Inlay hints with parameter names before the arguments of predicates and functions.
- Hints for unused types, constants, predicates, functions, objects and parameters.
