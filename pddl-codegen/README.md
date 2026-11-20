# pddl-codegen

Generates typed Rust planning code from PDDL, for the `#[pddl::task]`
attribute (the `pddl` crate's `codegen` feature) and for build scripts.

- `lifted(&domain)`: level B1. The domain is fixed when the crate compiles;
  problems load at run time.

The generated code refers to `::pddl::...`, so the crate that includes it needs
`pddl` with the `codegen` feature. See `docs/design/codegen.md` in the
repository for the layout of the generated code.

## License

Licensed under either of

- the European Union Public Licence 1.2 ([LICENSE-EUPL-1.2.md](../LICENSE-EUPL-1.2.md)), or
- the Mozilla Public License 2.0 ([LICENSE-MPL-2.0.txt](../LICENSE-MPL-2.0.txt)),

at your option.
