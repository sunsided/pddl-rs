# Changelog

All notable changes to the PDDL (pddl-lsp) extension are documented in this file.

## Unreleased

### Added

- Arguments of the wrong type are reported, such as a `truck` passed where a
  `place` is expected.
- Constructs the `:requirements` don't allow get a warning, such as `forall`
  in a precondition without `:universal-preconditions`.

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
