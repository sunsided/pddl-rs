# A PDDL 3.1 parser, strongly typed

A parser for the Planning Domain Definition Language version 3.1: written in Rust, based on [nom](https://crates.io/crates/nom).

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sunsided/pddl-rs/rust.yml)
![docs.rs](https://img.shields.io/docsrs/pddl)
![Crates.io](https://img.shields.io/crates/v/pddl)
![Crates.io](https://img.shields.io/crates/l/pddl)

Crate documentation is available on [docs.rs/pddl](https://docs.rs/pddl).

```toml
[dependencies]
pddl = "*"
```

The domain/problem types can be used independently of the parser; the parser
is however enabled by default via the `parser` crate feature.
To disable the parser and its dependencies, use

```toml
[dependencies]
pddl = { version = "*", default-features = false }
```

Documentation comments are assembled from the PDDL papers and [nergmada/planning-wiki](https://github.com/nergmada/planning-wiki).

## Usage Example

See [`tests/briefcase_world.rs`](tests/briefcase_world.rs) for the full example.

```rust
use pddl::{Problem, Parser};

pub const BRIEFCASE_WORLD_PROBLEM: &'static str = r#"
    (define (problem get-paid)
        (:domain briefcase-world)
        (:init (place home) (place office)
               (object p) (object d) (object b)
               (at B home) (at P home) (at D home) (in P))
        (:goal (and (at B office) (at D office) (at P home)))
    )
    "#;

fn main() {
    let problem = Problem::from_str(BRIEFCASE_WORLD_PROBLEM).unwrap();

    assert_eq!(problem.name(), "get-paid");
    assert_eq!(problem.domain(), "briefcase-world");
    assert!(problem.requirements().is_empty());
    assert_eq!(problem.init().len(), 9);
    assert_eq!(problem.goal().len(), 3);
}
```

### Caveat Emptor

At this point the parser supports all domain and problem definition
elements required to fully describe a PDDL 3.1 environment.
However, since types and enum variants are named closely to the underlying
BNF descriptions (see below), they may be a bit unwieldy to use still.

### Supported Elements

Parsers were implemented based on the BNF elements listed in the paper:

> "Complete BNF description of PDDL 3.1 (completely corrected)",
> _Daniel L. Kovacs_

See [ELEMENTS.md](ELEMENTS.md) for a graph of BNF elements.
