# Changelog

All notable changes to `pddl-lsp` will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Changed

- Licensed under `EUPL-1.2 OR MPL-2.0`, at your option, instead of `EUPL-1.2`
  alone. Versions already published keep the licence they were published under.
- `Server::request` and `Server::diagnostics` take `&mut self`: the server
  remembers which documents it published diagnostics for.

### Added

- The custom request `pddl/validatePlan` checks a plan against a domain and a
  problem, given as texts, with `pddl-validate`, and answers with a status,
  a message, the plan line it belongs to, repair advice and the metric. The
  VS Code extension's **PDDL: Validate Plan** uses it.
- The server asks clients that support it to watch `**/*.pddl`, so closed
  files changed on disk stay current in editors other than VS Code, and it
  re-reads changed closed files before a rename.
- PDDL+ `(:process ...)` and `(:event ...)` sections are analysed like
  actions: references, definitions, parameter scope, undeclared names, and
  section and keyword completion.

### Fixed

- A domain no longer sees the predicates, types or constants of another
  file's domain with the same name, in diagnostics, go to definition, rename
  and completion. A rename no longer edits that other file.
- Closing or deleting a document clears its diagnostics.
- An edit publishes diagnostics for open documents only, instead of one
  notification per file in the workspace.
- Rename refuses PDDL reserved words, such as `and` or `object`, and names
  already defined in the same scope.

## [0.2.0] - 2026-11-19

[0.2.0]: https://github.com/sunsided/pddl-rs/releases/tag/pddl-lsp-v0.2.0

### Added

- `pddl_lsp::server::run_stdio` serves stdin and stdout on the calling thread,
  so the server builds and runs on `wasm32-wasip1`, which has no threads. The
  binary uses it.
- Completion offers domain names in a problem's `(:domain ...)`, types in
  `(either ...)`, `minimize` and `maximize` after `:metric`, and functions and
  `total-time` in the metric expression. In `:init`, it offers only
  predicates, `not`, `at` and `=` after `(`, and only functions inside
  `(= ...)`.
- Arguments whose declared type doesn't fit the parameter are reported, such as
  `(at t1 t1)` where the second parameter of `at` is a `place`. Subtypes and
  `(either ...)` types fit; untyped names and types the workspace doesn't know
  are not checked.
- Constructs the `:requirements` don't allow get a warning, such as `forall` in a
  precondition without `:universal-preconditions`, `when` without
  `:conditional-effects`, or typed names without `:typing`. Shorthands such as
  `:adl` and `:fluents` are expanded, and a problem also has its domain's
  requirements. A quick fix adds the missing requirement, creating the
  `(:requirements ...)` section if there is none.
- Goals that can never hold are reported: a literal of a static predicate,
  one that no action changes, that `:init` decides against, such as
  `(road a b)` missing from `:init`. Static predicates are marked in hover and
  carry the `readonly` semantic token modifier.

## [0.1.0] - 2026-11-14

[0.1.0]: https://github.com/sunsided/pddl-rs/releases/tag/pddl-lsp-v0.1.0

### Added

- A language server for PDDL over stdio: diagnostics for syntax errors, unknown names, wrong
  argument counts, undeclared variables and duplicate definitions, checked across domain and
  problem files; hover, go to definition, references, rename, document and workspace symbols,
  completion, signature help, formatting, quick fixes, folding and semantic highlighting.
- Inlay hints with parameter names before the arguments of predicates and functions.
- Hints for unused types, constants, predicates, functions, objects and parameters.
