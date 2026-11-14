//! Converts a PDDL domain and problem to JSON, and reads a problem written in JSON.
//!
//! Run with `cargo run --example as_json --features serde`.

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

/// The same problem, written in JSON by hand.
const PROBLEM_JSON: &str = r#"{
  "problem": "deliver",
  "domain": "logistics",
  "objects": {"t1": "truck", "depot": "location", "shop": "location"},
  "init": [
    ["at", "t1", "depot"],
    {"=": [["distance", "depot", "shop"], 5]},
    {"=": [["total-cost"], 0]}
  ],
  "goal": ["at", "t1", "shop"],
  "metric": {"minimize": ["total-cost"]}
}"#;

fn main() {
    let domain = Domain::from_str(DOMAIN).expect("the domain is valid PDDL");
    let problem = Problem::from_str(PROBLEM).expect("the problem is valid PDDL");

    let json = serde_json::to_string_pretty(&domain).expect("a domain serializes");
    println!("{json}\n");
    let back: Domain = serde_json::from_str(&json).expect("the JSON describes a domain");
    assert_eq!(back, domain);

    let written: Problem =
        serde_json::from_str(PROBLEM_JSON).expect("the JSON describes a problem");
    assert_eq!(written, problem);
    println!(
        "{}",
        serde_json::to_string_pretty(&written).expect("a problem serializes")
    );
}
