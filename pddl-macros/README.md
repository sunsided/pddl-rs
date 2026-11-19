# pddl-macros

Compile-time checked PDDL for the [`pddl`](https://crates.io/crates/pddl) crate.
The macros parse your PDDL while the crate compiles, so a syntax error fails
the build instead of a run. At run time, the PDDL is parsed once, on first use.

```toml
[dependencies]
pddl = "0.4"
pddl-macros = "0.1"
```

```rust
use std::sync::LazyLock;
use pddl::{Domain, Problem};

static DOMAIN: LazyLock<Domain> = pddl_macros::domain_file!("pddl/blocks-domain.pddl");
static PROBLEM: LazyLock<Problem> = pddl_macros::problem!(r#"
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
  its `parser` feature, which is on by default. Use the `pddl` version that
  `pddl-macros` was released with.

Requires Rust 1.88 or newer.

## License

Licensed under the [European Union Public Licence 1.2](../LICENSE.md).
