# pddl-codegen

Generates typed Rust planning code from PDDL, for the `#[pddl::task]`
attribute (the `pddl` crate's `codegen` feature) and for build scripts.

- `lifted(&domain)`: level B1. The domain is fixed when the crate compiles;
  problems load at run time.

The generated code refers to `::pddl::...`, so the crate that includes it needs
`pddl` with the `codegen` feature. See `docs/design/codegen.md` in the
repository for the layout of the generated code.
