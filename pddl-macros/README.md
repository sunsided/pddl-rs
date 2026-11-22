# pddl-macros

Compile-time checked PDDL for the [`pddl`](https://crates.io/crates/pddl) crate.
The macros parse your PDDL while the crate compiles, so a syntax error fails
the build instead of a run. At run time, the PDDL is parsed once, on first use.

With the `codegen` feature, the `#[task]` attribute also turns a domain into
typed Rust planning code (see
[`pddl-codegen`](https://github.com/sunsided/pddl-rs/tree/main/pddl-codegen)).

Part of the [planning and search crates](https://sunsided.github.io/planning-and-search/).

## Who this is for

- Rust developers who embed PDDL domains and problems in their program, as
  inline literals or files next to the source, and want a typo in the PDDL to
  fail the build with a pointer to the offending line.
- Tools and tests that need a parsed `Domain` or `Problem` in a `static`
  without writing the parse-and-unwrap boilerplate.
- To parse PDDL that only arrives at run time, use the parser in
  [`pddl`](https://github.com/sunsided/pddl-rs) directly; to check plans
  against a domain, use [`pddl-validate`](https://github.com/sunsided/pddl-rs/tree/main/pddl-validate).

## Installation

Use the macros through the `pddl` crate's `macros` feature, which re-exports them:

```toml
[dependencies]
pddl = { version = "0.5", features = ["macros"] }
```

## Usage

```rust
use std::sync::LazyLock;
use pddl::{Domain, Problem};

static DOMAIN: LazyLock<Domain> = pddl::domain_file!("pddl/blocks-domain.pddl");
static PROBLEM: LazyLock<Problem> = pddl::problem!(r#"
    (define (problem p1) (:domain blocks)
      (:objects a b) (:init (clear a) (clear b)) (:goal (on a b)))
"#);
```

| Macro | Input | Expands to |
|---|---|---|
| `domain!` / `domain_file!` | one `(define (domain ...))` | `LazyLock<pddl::Domain>` |
| `problem!` / `problem_file!` | one `(define (problem ...))` | `LazyLock<pddl::Problem>` |
| `pddl!` / `pddl_file!` | any number of definitions | `LazyLock<pddl::PddlFile>` |
| `validated!` / `validated_file!` | any number of definitions | the checked source, `&'static str` |

- The inline forms take one string literal. The `_file` forms take a path
  relative to the calling file, like `include_str!`. Cargo rebuilds the crate
  when the file changes.
- Every syntax error is reported in the same build. Each message points at the
  PDDL file's line and column, or at the line in your `.rs` file for an inline
  literal, and shows the offending line:

  ```text
  error: src/main.rs:6:7: unknown section: unexpected `(:predicats (on ?s)))`
           = help: did you mean `:predicates`?
           |
         3 |       (:predicats (on ?s)))
           |       ^~~~~~~~~~~
  ```

- The expansion refers to `::pddl`, so `pddl` must be a direct dependency with
  its `parser` feature, which is on by default. Depending on `pddl-macros`
  directly works too, next to `pddl`.
- `pddl-macros` checks with `pddl-core`, the crate behind `pddl`. Cargo builds
  one `pddl-core` for the macros and your program, so the parser that checks at
  compile time is the one that parses at run time.

Requires Rust 1.88 or newer.

## License

Licensed under either of

- the European Union Public Licence 1.2 ([LICENSE-EUPL-1.2.md](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-EUPL-1.2.md)), or
- the Mozilla Public License 2.0 ([LICENSE-MPL-2.0.txt](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-MPL-2.0.txt)),

at your option.
