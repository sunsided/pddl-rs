//! Demonstrates the `pretty` feature.
//!
//! Run with: `cargo run --example pretty_print --features pretty`

use pddl::{Name, Parser, Pretty, Type, Variable};

fn main() {
    let name = Name::new("location");
    let var = Variable::from("x");
    let simple = Type::from("truck");
    let either = Type::from_iter(["car", "truck", "motorbike"]);

    println!("== trivial cases (width 80) ==");
    println!("{}", name.pretty(80));
    println!("{}", var.pretty(80));
    println!("{}", simple.pretty(80));

    println!("\n== (either …) at different widths ==");
    for width in [80, 20, 12, 8] {
        println!("-- width = {width}");
        println!("{}", either.pretty(width));
    }

    println!("\n== durative action (width 80) ==");
    let durative_action = r#"(define (domain rover)
  (:requirements :strips :typing :durative-actions)
  (:types location rover)
  (:predicates (at ?r - rover ?l - location))
  (:durative-action navigate
    :parameters (?r - rover ?from ?to - location)
    :duration (= ?duration 10)
    :condition (and (at start (at ?r ?from)))
    :effect (and (at end (at ?r ?to)) (at end (not (at ?r ?from)))))
)"#;
    let domain = pddl::Domain::from_str(durative_action).unwrap();
    println!("{}", domain.pretty(80));
}
