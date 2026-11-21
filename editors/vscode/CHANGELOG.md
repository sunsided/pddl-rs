# Changelog

All notable changes to the PDDL (pddl-lsp) extension are documented in this file.

## Unreleased

### Changed

- Licensed under `EUPL-1.2 OR MPL-2.0`, at your option, instead of `EUPL-1.2`
  alone. Versions already published keep the licence they were published under.
- Needs VS Code 1.146 or later, up from 1.91, and uses the stable
  `vscode-languageclient` 11 instead of a 10.0 pre-release.

### Added

- **PDDL: Solve with Planning-as-a-Service** solves the open problem on the
  server of the new `pddl.planningAsAService.url` setting. Off by default; it
  asks before it sends the domain and problem to a server.
- Plan files (`.plan`, `sas_plan`) are highlighted, and Go to Definition on
  a step's action opens it in the domain.
- **PDDL: Validate Plan** checks the open plan with the built-in validator,
  `pddl-validate`, through the language server: on the desktop and on the
  web, with unsaved edits, and with the validator's message and repair hint
  on the failing step.
- **PDDL: Validate Plan with VAL** runs VAL (the new `pddl.validator`
  setting) on the open plan and shows its findings on the failing step.
  Desktop only.
- **PDDL: Solve with planutils** runs the planner of the new `pddl.planner`
  setting (`lama-first` by default) on the open problem and its domain, as a
  task. Desktop only.
- A smoke test (`yarn run test:web`) runs the web entry point in VS Code for
  the Web in a headless Chromium and checks that the bundled server publishes
  diagnostics. It runs in CI.

## 0.2.0 - 2026-11-19

### Added

- The extension works in VS Code for the Web, such as vscode.dev and
  github.dev, with the bundled server.
- Snippets for `:functions`, `:constraints`, `:metric`, `imply`,
  `preference`, `increase`, `decrease`, `at start`, `at end`, `over all` and
  timed initial literals.
- Completion offers domain names in `(:domain ...)`, types in `(either ...)`,
  and `minimize` and `maximize` in `:metric`.
- Arguments of the wrong type are reported, such as a `truck` passed where a
  `place` is expected.
- Constructs the `:requirements` don't allow get a warning, such as `forall`
  in a precondition without `:universal-preconditions`, with a quick fix that
  adds the requirement.
- Goals that can never hold are reported, such as `(road a b)` missing from
  `:init` when no action changes `road`. Hover marks such predicates as static.

## 0.1.1 - 2026-11-14

### Added

- An icon for the Marketplace.
- Inlay hints with parameter names before the arguments of predicates and
  functions.
- Unused types, constants, predicates, functions, objects and parameters are
  shown faded.
- Snippets for a domain, problem, action, durative action, derived predicate,
  `forall`, `exists` and `when`.

### Fixed

- In Restricted Mode, the extension no longer tries to start the language
  server, which failed because VS Code disables the WASI runtime there. Only
  highlighting is available until you trust the folder; the server then starts
  on its own.
- A workspace can only set `pddl.server.path` once you trust it.

## 0.1.0 - 2026-11-14

- First release: the `pddl-lsp` language server, bundled as WebAssembly, with
  diagnostics, navigation, rename, completion, signature help, formatting,
  quick fixes and semantic highlighting.
- Syntax highlighting for PDDL files, for ` ```pddl ` blocks in Markdown, and
  for strings marked with a `language=pddl` comment in Rust, JavaScript,
  TypeScript and Python.
