//! Calls the generator directly: the macro runs it inside rustc, where test
//! coverage doesn't see it. `pddl-bench` compiles the same domain and runs it.

use pddl_codegen::{grounded, lifted, Limits};
use pddl_core::{Domain, Parser, Problem};

/// The generated code for the kitchen-sink domain, as text.
fn kitchen_sink() -> String {
    let domain = Domain::from_str(include_str!("fixtures/kitchen-sink.pddl")).unwrap();
    lifted(&domain).unwrap().to_string()
}

#[test]
fn rare_constructs_generate() {
    let code = kitchen_sink();
    for expected in [
        "pub struct State",
        ":: pddl :: runtime :: BitTable",
        "pub type_ :",
        "pub self_ :",
        "pub struct Self_",
        "fn derive",
    ] {
        assert!(code.contains(expected), "missing `{expected}`");
    }
}

#[test]
fn grounded_tasks_reject_what_masks_cannot_hold() {
    let domain = Domain::from_str(
        "(define (domain d) (:requirements :negative-preconditions :conditional-effects)
           (:predicates (p ?x) (q))
           (:action a :parameters (?x) :precondition (not (p ?x)) :effect (p ?x))
           (:action b :parameters () :precondition (q) :effect (when (q) (not (q)))))",
    )
    .unwrap();
    let problem = Problem::from_str(
        "(define (problem pr) (:domain d) (:objects o1 o2) (:init (q)) (:goal (and (p o1) (not (q)))))",
    )
    .unwrap();
    let error = grounded(&domain, &problem, &Limits::default()).unwrap_err();
    assert!(
        error.message().contains("conditional effects"),
        "{}",
        error.message()
    );

    let limits = Limits {
        max_facts: 1,
        ..Limits::default()
    };
    let domain = Domain::from_str(
        "(define (domain d) (:requirements :negative-preconditions) (:predicates (p ?x))
           (:action a :parameters (?x) :precondition (not (p ?x)) :effect (p ?x)))",
    )
    .unwrap();
    let problem = Problem::from_str(
        "(define (problem pr) (:domain d) (:objects o1 o2) (:init) (:goal (not (p o1))))",
    )
    .unwrap();
    let error = grounded(&domain, &problem, &limits).unwrap_err();
    assert!(
        error
            .message()
            .starts_with("the grounded task has 2 facts, more than max_facts = 1"),
        "{}",
        error.message()
    );
    let code = grounded(&domain, &problem, &Limits::default())
        .unwrap()
        .to_string();
    assert!(code.contains("pub static OPERATORS"));
}

#[test]
fn action_names_that_clash_are_errors() {
    let domain = Domain::from_str(
        "(define (domain d) (:action go-on :parameters () :precondition () :effect ())
           (:action go_on :parameters () :precondition () :effect ()))",
    )
    .unwrap();
    let error = lifted(&domain).unwrap_err();
    assert!(
        error.message().contains("action names `go-on` and `go_on`"),
        "{}",
        error.message()
    );
    let domain = Domain::from_str(
        "(define (domain d) (:action a :parameters (?x-y ?x_y) :precondition () :effect ()))",
    )
    .unwrap();
    let error = lifted(&domain).unwrap_err();
    assert!(
        error.message().starts_with("action `a`: "),
        "{}",
        error.message()
    );
}
