# pddl-codegen

Generates typed Rust planning code from PDDL, for the `#[pddl::task]`
attribute (the `pddl` crate's `codegen` feature) and for build scripts. A
domain becomes a module with one id type per PDDL type, a `State` with one
field per predicate that actions change, `Statics` for the predicates they
don't, a `Copy` enum `Action` with typed fields, and a `Task` that implements
`pddl_runtime::Task`. Successor generation is nested loops over the action
parameters that check each precondition as soon as its variables are bound, so
a planner over the generated `Task` monomorphizes per domain.

- `lifted(&domain)`: the domain is fixed when the crate compiles; problems
  load at run time with `Task::new(&problem)`. Supports STRIPS, `:typing`,
  `:negative-preconditions`, `:equality`, `:adl` (`or`, `imply`, `exists`,
  `forall`, `not` over any formula, and `forall` and `when` in effects),
  numeric fluents with action costs, and stratified derived predicates.
- `lifted_with(&domain, &bindings)`: as `lifted`, with PDDL types bound to
  Rust types and static predicates and functions attached to pure Rust
  functions.
- `grounded(&domain, &problem, &limits)`: domain and problem are both fixed;
  the task is grounded at build time into bit masks over a `[u64; WORDS]`
  state. STRIPS with negative preconditions only; `Limits` turns a too large
  task into an error instead of a slow build.

Anything unsupported is an `Error` that names the construct.

Part of the [planning and search crates](https://sunsided.github.io/planning-and-search/).

## Who this is for

- Rust developers who plan in one known PDDL domain (a game, a robot cell, a
  logistics model) and want the search to run on typed, compiled code rather
  than an interpreted grounding.
- People benchmarking planners, who can compare generated code against a
  runtime grounder on the same domain.
- Most users get this crate through `#[pddl::task]` in the
  [`pddl`](https://github.com/sunsided/pddl-rs) crate; depend on
  `pddl-codegen` directly only to generate code from a build script. To parse
  PDDL and plan over it at run time instead, see
  [`miniplan`](https://github.com/sunsided/miniplan).

## Installation

Through the attribute:

```toml
[dependencies]
pddl = { version = "0.5", features = ["codegen"] }
```

From a build script, which writes the tokens to `OUT_DIR`:

```toml
[dependencies]
pddl = { version = "0.5", features = ["codegen"] }

[build-dependencies]
pddl = "0.5"
pddl-codegen = "0.1"
```

The generated code refers to `::pddl::...`, so the crate that includes it needs
`pddl` with the `codegen` feature. See
[`docs/design/codegen.md`](https://github.com/sunsided/pddl-rs/blob/main/docs/design/codegen.md) for the layout of the
generated code.

## License

Licensed under either of

- the European Union Public Licence 1.2 ([LICENSE-EUPL-1.2.md](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-EUPL-1.2.md)), or
- the Mozilla Public License 2.0 ([LICENSE-MPL-2.0.txt](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-MPL-2.0.txt)),

at your option.
