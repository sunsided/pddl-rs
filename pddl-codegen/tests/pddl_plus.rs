//! Pins that code generation rejects PDDL+ processes and events.

use pddl_codegen::lifted;
use pddl_core::{Domain, Parser};

#[test]
fn processes_and_events_are_unsupported() {
    for section in [
        "(:process grow :parameters () :effect (increase (f) (* #t 1)))",
        "(:event flip :parameters () :precondition (q) :effect (not (q)))",
    ] {
        let source = format!("(define (domain d) (:predicates (q)) (:functions (f)) {section})");
        let domain = Domain::from_str(&source).unwrap();
        let error = lifted(&domain).unwrap_err();
        assert_eq!(
            error.message(),
            "unsupported: processes and events",
            "{section}"
        );
    }
}
