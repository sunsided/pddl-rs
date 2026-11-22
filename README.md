# A PDDL 3.1 parser, strongly typed

A parser for the Planning Domain Definition Language version 3.1, written in Rust. It reads domains and problems into a strongly typed syntax tree whose types follow the BNF of the PDDL 3.1 specification one to one, and reports every syntax error in a file with its position, not only the first one. Around that, optional features pretty-print the tree back to PDDL (round-trip safe), serialize it with serde, ground a domain and problem into a Fast Downward SAS+ task, check PDDL at compile time, and generate typed Rust planning code from a domain. A language server and an independent plan validator live in the same repository.

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sunsided/pddl-rs/rust.yml)
![docs.rs](https://img.shields.io/docsrs/pddl)
[![codecov](https://codecov.io/gh/sunsided/pddl-rs/graph/badge.svg?token=8H8K6HW6AF)](https://codecov.io/gh/sunsided/pddl-rs)
![Crates.io](https://img.shields.io/crates/v/pddl)
![Crates.io](https://img.shields.io/crates/l/pddl)

Part of the [planning and search crates](https://sunsided.github.io/planning-and-search/); the workspace's plan validator, pddl-validate, has its own [overview page](https://sunsided.github.io/planning-and-search/crates/pddl-validate/).

Crate documentation is available on [docs.rs/pddl](https://docs.rs/pddl).

## Who this is for

- Planner and tool authors who need to read PDDL in Rust and would rather match on typed goal descriptions and effects than walk an S-expression tree.
- People writing or teaching PDDL, who get a language server for their editor ([`pddl-lsp`](https://github.com/sunsided/pddl-rs/tree/main/pddl-lsp)) and a validator that explains why a plan fails ([`pddl-validate`](https://github.com/sunsided/pddl-rs/tree/main/pddl-validate)).
- Rust developers who ship PDDL with their program and want syntax errors to fail the build, or a domain compiled into typed planning code instead of interpreted at run time.
- Researchers who want Fast Downward's `output.sas` format from PDDL without leaving Rust, for example to feed [`sas-plus`](https://github.com/sunsided/sas-plus) heuristics.
- If you want a planner rather than a parser, [`miniplan`](https://github.com/sunsided/miniplan) grounds and solves PDDL tasks on top of this crate.

## Installation

```toml
[dependencies]
pddl = "0.5"
```

The domain/problem types can be used independently of the parser; the parser
is however enabled by default via the `parser` crate feature.
To disable the parser and its dependencies, use

```toml
[dependencies]
pddl = { version = "0.5", default-features = false }
```

The types and the parser live in `pddl-core`, which `pddl` re-exports. Depend on `pddl`, not on `pddl-core`.

Documentation comments are assembled from the PDDL papers and [nergmada/planning-wiki](https://github.com/nergmada/planning-wiki).

## Editor Support

[`pddl-lsp`](https://github.com/sunsided/pddl-rs/tree/main/pddl-lsp) is a language server built on this crate. It works in
VS Code, Neovim, Vim, Helix, Emacs, Sublime Text and other editors that speak
the Language Server Protocol. It reports every syntax error in a file plus
unknown names and wrong argument counts, and checks problems against their
domain across files. It also provides hover, go to definition, references,
rename, completion, signature help and formatting.

```shell
cargo install --locked pddl-lsp
```

Then set up your editor as described in the [`pddl-lsp` README](https://github.com/sunsided/pddl-rs/tree/main/pddl-lsp#editor-setup).
For VS Code, use the extension in [`editors/vscode`](https://github.com/sunsided/pddl-rs/tree/main/editors/vscode), which bundles the server
compiled to WebAssembly.

The parser exposes what the server builds on:
[`pddl::syntax`](https://docs.rs/pddl/latest/pddl/syntax/index.html) gives the
position of every atom, list and comment, and
[`pddl::parsers::check`](https://docs.rs/pddl/latest/pddl/parsers/fn.check.html)
reports every error in a file rather than only the first.

## Compile-time Checked PDDL

The `macros` feature checks PDDL while your crate compiles and turns syntax
errors into compile errors. It needs Rust 1.88.

```toml
[dependencies]
pddl = { version = "0.5", features = ["macros"] }
```

```rust ignore
static DOMAIN: LazyLock<Domain> = pddl::domain_file!("blocks-domain.pddl");
```

The macros come from [`pddl-macros`](https://github.com/sunsided/pddl-rs/tree/main/pddl-macros); see its
[README](https://github.com/sunsided/pddl-rs/tree/main/pddl-macros#readme) for all of them. The types and the parser
live in `pddl-core`, which `pddl` re-exports, so that `pddl-macros` can use the
parser without a dependency cycle.

## Generated Planning Code

The `codegen` feature adds the `#[pddl::task]` attribute, which turns a domain
into a module of typed Rust planning code: one id type per PDDL type, a `State`
with one field per predicate that actions change, an `Action` enum and a `Task` that implements
`pddl::runtime::Task`, so planners monomorphize per domain. It needs Rust 1.88.

```toml
[dependencies]
pddl = { version = "0.5", features = ["codegen"] }
```

```rust ignore
#[pddl::task(domain = "blocks.pddl")]
mod blocks {}
```

The generator is [`pddl-codegen`](https://github.com/sunsided/pddl-rs/tree/main/pddl-codegen), which build scripts can
also call directly, and the generated code builds on
[`pddl-runtime`](https://github.com/sunsided/pddl-rs/tree/main/pddl-runtime).

## Usage Example

See [`tests/briefcase_world.rs`](https://github.com/sunsided/pddl-rs/blob/main/tests/briefcase_world.rs) for the full example.

### Parsing a Single Problem

```rust
use pddl::{Problem, Parser};

pub const BRIEFCASE_WORLD_PROBLEM: &'static str = 
    // language=pddl
    r#"
    (define (problem get-paid)
        (:domain briefcase-world)
        (:init (place home) (place office)                  ; place types
               (object p) (object d) (object b)             ; object types
               (at B home) (at P home) (at D home) (in P))  ; setup
        (:goal (and (at B office) (at D office) (at P home)))
    )
    "#;

fn main() {
    let problem = Problem::from_str(BRIEFCASE_WORLD_PROBLEM).unwrap();

    assert_eq!(problem.name(), "get-paid");
    assert_eq!(problem.domain(), "briefcase-world");
    assert!(problem.requirements().is_empty());
    assert_eq!(problem.init().len(), 9);
    assert_eq!(problem.goals().len(), 3);
}
```

### Parsing a File with Multiple Domains and Problems

PDDL files often contain multiple `(define ...)` blocks — domains and problems
in any order. Use `PddlFile` to parse them all at once:

```rust
use pddl::{PddlFile, Parser};

pub const PDDL_FILE: &'static str = 
    // language=pddl
    r#"
    (define (domain blocks-world)
        (:requirements :strips)
        (:predicates (clear ?x) (on ?x ?y) (on-table ?x))
        ; ... actions omitted for brevity
    )

    (define (problem blocks-problem-1)
        (:domain blocks-world)
        (:init (on-table a) (on b a) (clear b))
        (:goal (and (on a b)))
    )

    (define (problem blocks-problem-2)
        (:domain blocks-world)
        (:init (on-table a) (on-table b) (clear a) (clear b))
        (:goal (and (on b a)))
    )
    "#;

fn main() {
    let file = PddlFile::from_str(PDDL_FILE).unwrap();

    assert_eq!(file.domain_count(), 1);
    assert_eq!(file.problem_count(), 2);

    let domain = &file.domains[0];
    assert_eq!(domain.name(), "blocks-world");
    assert_eq!(domain.predicates().len(), 3);

    let problem1 = &file.problems[0];
    assert_eq!(problem1.name(), "blocks-problem-1");
    assert_eq!(problem1.goals().len(), 1);
}
```

### Grounding to SAS+

The `sas-plus` feature grounds a domain and problem into a
[`sas-plus`](https://crates.io/crates/sas-plus) task in Fast Downward's
`output.sas` format, without leaving Rust:

```toml
[dependencies]
pddl = { version = "0.5", features = ["sas-plus"] }
```

```rust ignore
let file = pddl::sas_plus::ground(&domain, &problem)?;
print!("{file}");             // output.sas, ready for Fast Downward's search
let task = file.to_task();    // a plain sas_plus::Task for heuristics
```

Every reachable fluent atom becomes a binary variable. Fast Downward's
translator instead merges mutually exclusive atoms into multi-valued variables.
The grounder supports STRIPS with typing, negative preconditions, equality,
universal and existential preconditions, conditional effects and action costs.
It rejects other constructs with an error instead of dropping them. See the
[`sas_plus` module docs](https://docs.rs/pddl/latest/pddl/sas_plus/index.html).

### Validating Plans

[`pddl-validate`](https://github.com/sunsided/pddl-rs/tree/main/pddl-validate), a separate crate in this repository, checks
plans against a domain and problem, independently of any planner, and explains
the first step that fails in the style of VAL's repair advice. See its
[README](https://github.com/sunsided/pddl-rs/tree/main/pddl-validate#readme) for the supported PDDL levels.

### Caveat Emptor

The parser supports all domain and problem definition elements required to
fully describe a PDDL 3.1 environment. Since types and enum variants are named
closely after the underlying BNF descriptions (see below), some of them are a
bit unwieldy to use.

### Supported Elements

Parsers were implemented based on the BNF elements listed in the paper:

> "Complete BNF description of PDDL 3.1 (completely corrected)",
> _Daniel L. Kovacs_

See [ELEMENTS.md](https://github.com/sunsided/pddl-rs/blob/main/ELEMENTS.md) for a graph of BNF elements.

## License

Licensed under either of

- the European Union Public Licence 1.2 ([LICENSE-EUPL-1.2.md](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-EUPL-1.2.md)), or
- the Mozilla Public License 2.0 ([LICENSE-MPL-2.0.txt](https://github.com/sunsided/pddl-rs/blob/main/LICENSE-MPL-2.0.txt)),

at your option.
