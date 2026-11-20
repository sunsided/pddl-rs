//! Malformed domains and problems fail [`Validator::new`] before any plan runs.

use pddl::{Domain, Parser, Problem};
use pddl_validate::{ModelError, ModelErrorKind, Validator};

/// Builds a validator for a domain and problem that must not be well-formed.
fn model_error(domain: &str, problem: &str) -> ModelError {
    let domain = Domain::from_str(domain).expect("the domain parses");
    let problem = Problem::from_str(problem).expect("the problem parses");
    Validator::new(&domain, &problem).expect_err("the task is malformed")
}

/// A domain whose single action has `body` as precondition and effect.
fn domain(body: &str) -> String {
    format!(
        "(define (domain d) (:requirements :typing) (:types block)
           (:constants table - block)
           (:predicates (on ?x ?y - block) (clear ?x - block))
           {body})"
    )
}

/// A problem for [`domain`] with `goal`.
fn problem(goal: &str) -> String {
    format!(
        "(define (problem p) (:domain d) (:objects a b - block) (:init (clear a)) (:goal {goal}))"
    )
}

#[test]
fn problem_for_another_domain() {
    let error = model_error(
        &domain(""),
        &problem("(clear a)").replace("(:domain d)", "(:domain other)"),
    );
    assert_eq!(
        error.kind,
        ModelErrorKind::DomainMismatch {
            domain: "d".into(),
            problem_domain: "other".into()
        }
    );
}

#[test]
fn unknown_predicate() {
    let error = model_error(&domain(""), &problem("(heavy a)"));
    assert_eq!(error.location, "goal");
    assert_eq!(error.kind, ModelErrorKind::UnknownPredicate("heavy".into()));
}

#[test]
fn wrong_arity() {
    let action = "(:action a :parameters (?x - block) :precondition (on ?x) :effect (clear ?x))";
    let error = model_error(&domain(action), &problem("(clear a)"));
    assert_eq!(error.location, "action `a`");
    assert_eq!(
        error.kind,
        ModelErrorKind::Arity {
            symbol: "on".into(),
            expected: 2,
            found: 1
        }
    );
    assert_eq!(
        error.to_string(),
        "action `a`: `on` takes 2 arguments but is given 1"
    );
}

#[test]
fn unknown_object_in_goal() {
    let error = model_error(&domain(""), &problem("(clear c)"));
    assert_eq!(error.kind, ModelErrorKind::UnknownObject("c".into()));
}

#[test]
fn unbound_variable() {
    let action = "(:action a :parameters (?x - block) :precondition (clear ?y) :effect (clear ?x))";
    let error = model_error(&domain(action), &problem("(clear a)"));
    assert_eq!(error.kind, ModelErrorKind::UnboundVariable("y".into()));
}

#[test]
fn unknown_type() {
    let action = "(:action a :parameters (?x - brick) :effect (clear ?x))";
    let error = model_error(&domain(action), &problem("(clear a)"));
    assert_eq!(error.kind, ModelErrorKind::UnknownType("brick".into()));

    let objects = problem("(clear a)").replace("a b - block", "a b - brick");
    let error = model_error(&domain(""), &objects);
    assert_eq!(error.kind, ModelErrorKind::UnknownType("brick".into()));
}

#[test]
fn duplicate_action() {
    let actions = "(:action a :parameters () :effect (clear table))
                   (:action A :parameters () :effect (clear table))";
    let error = model_error(&domain(actions), &problem("(clear a)"));
    assert_eq!(error.kind, ModelErrorKind::DuplicateAction("a".into()));
}

#[test]
fn setting_a_derived_predicate() {
    let body = "(:derived (clear ?x - block) (not (on table ?x)))
                (:action a :parameters () :effect (clear table))";
    let error = model_error(
        &domain(body),
        &problem("(clear a)").replace("(:init (clear a))", "(:init)"),
    );
    assert_eq!(
        error.kind,
        ModelErrorKind::DerivedPredicateSet("clear".into())
    );

    let body = "(:derived (clear ?x - block) (not (on table ?x)))";
    let error = model_error(&domain(body), &problem("(clear a)"));
    assert_eq!(error.location, "init");
    assert_eq!(
        error.kind,
        ModelErrorKind::DerivedPredicateSet("clear".into())
    );
}

#[test]
fn setting_equality() {
    let action = "(:action a :parameters (?x ?y - block) :effect (= ?x ?y))";
    let error = model_error(&domain(action), &problem("(clear a)"));
    assert_eq!(error.kind, ModelErrorKind::EqualitySet);
}
