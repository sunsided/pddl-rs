//! Converts a PDDL domain and problem to NOVA, and reads a problem written in NOVA.
//!
//! Run with `cargo run --example as_nova --features serde`.

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

/// The same problem, written in NOVA by hand.
const PROBLEM_NOVA: &str = "\
problem: deliver
domain: logistics
objects: {t1: truck, depot: location, shop: location}
init:
  - [at, t1, depot]
  - \"=\": ([distance, depot, shop], 5)
  - \"=\": ([total-cost], 0)
goal: [at, t1, shop]
metric:
  minimize: [total-cost]
";

fn main() {
    let domain = Domain::from_str(DOMAIN).expect("the domain is valid PDDL");
    let problem = Problem::from_str(PROBLEM).expect("the problem is valid PDDL");

    let nova = novafile::serde::to_string(&domain).expect("a domain serializes");
    println!("{nova}");
    let back: Domain = novafile::serde::from_str(&nova).expect("the NOVA describes a domain");
    assert_eq!(back, domain);

    let written: Problem =
        novafile::serde::from_str(PROBLEM_NOVA).expect("the NOVA describes a problem");
    assert_eq!(written, problem);
    print!(
        "{}",
        novafile::serde::to_string(&written).expect("a problem serializes")
    );
}
