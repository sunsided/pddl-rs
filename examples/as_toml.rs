//! Converts a PDDL domain and problem to TOML, and reads a problem written in TOML.
//!
//! Run with `cargo run --example as_toml --features serde`.
//!
//! Parameter order is key order, so `toml` needs its `preserve_order` feature:
//! without it, the serializer sorts the keys.

use pddl::{Domain, Parser, Problem};

/// A small logistics domain.
const DOMAIN: &str = "(define (domain logistics)
  (:requirements :strips :typing :action-costs)
  (:types truck location)
  (:predicates (at ?t - truck ?l - location))
  (:functions (distance ?a ?b - location) (total-cost))
  (:action drive
    :parameters (?t - truck ?from ?to - location)
    :precondition (and (at ?t ?from) (not (= ?from ?to)))
    :effect (and (not (at ?t ?from)) (at ?t ?to)
                 (increase (total-cost) (distance ?from ?to)))))";

/// A problem for it.
const PROBLEM: &str = "(define (problem deliver) (:domain logistics)
  (:objects t1 - truck depot shop - location)
  (:init (at t1 depot) (= (distance depot shop) 5) (= (total-cost) 0))
  (:goal (at t1 shop))
  (:metric minimize (total-cost)))";

/// The same problem, written in TOML by hand.
const PROBLEM_TOML: &str = r#"
problem = "deliver"
domain = "logistics"
goal = ["at", "t1", "shop"]
init = [
  ["at", "t1", "depot"],
  { "=" = [["distance", "depot", "shop"], 5] },
  { "=" = [["total-cost"], 0] },
]

[objects]
t1 = "truck"
depot = "location"
shop = "location"

[metric]
minimize = ["total-cost"]
"#;

fn main() {
    let domain = Domain::from_str(DOMAIN).expect("the domain is valid PDDL");
    let problem = Problem::from_str(PROBLEM).expect("the problem is valid PDDL");

    let toml = toml::to_string(&domain).expect("a domain serializes");
    println!("{toml}");
    let back: Domain = toml::from_str(&toml).expect("the TOML describes a domain");
    assert_eq!(back, domain);

    let written: Problem = toml::from_str(PROBLEM_TOML).expect("the TOML describes a problem");
    assert_eq!(written, problem);
    print!(
        "{}",
        toml::to_string(&written).expect("a problem serializes")
    );
}
