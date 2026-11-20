//! Constructs the validator does not interpret make every report `Unsupported`, never
//! valid.

use pddl_validate::{Construct, ValidationStatus};

use crate::Task;

/// Asserts that every plan for the task is unsupported because of `construct`.
fn assert_unsupported(domain: &str, problem: &str, plan: &str, construct: &Construct) {
    let report = Task::new(domain, problem).run(plan);
    assert_eq!(report.status(), ValidationStatus::Unsupported, "{report:?}");
    assert_eq!(report.unsupported().unwrap().construct, *construct);
    assert_eq!(report.trace(), []);
}

/// A one-predicate domain with `extra` appended to its body.
fn domain(extra: &str) -> String {
    format!("(define (domain d) (:requirements :strips) (:predicates (p) (q)) (:action a :parameters () :effect (p)) {extra})")
}

/// A problem for [`domain`] with `extra` appended to its body.
fn problem(extra: &str) -> String {
    format!("(define (problem x) (:domain d) (:init) (:goal (p)) {extra})")
}

#[test]
fn durative_actions() {
    let action = "(:durative-action go :parameters () :duration (= ?duration 1)
                    :condition (at start (q)) :effect (at end (p)))";
    assert_unsupported(
        &domain(action),
        &problem(""),
        "(a)",
        &Construct::DurativeActions,
    );
}

#[test]
fn temporal_plans() {
    assert_unsupported(
        &domain(""),
        &problem(""),
        "0.000: (a) [1.000]",
        &Construct::TemporalPlans,
    );
    assert_unsupported(
        &domain(""),
        &problem(""),
        "(a) [1]",
        &Construct::TemporalPlans,
    );
}

#[test]
fn timed_initial_literals() {
    let problem = problem("").replace("(:init)", "(:init (at 10 (q)))");
    assert_unsupported(
        &domain(""),
        &problem,
        "(a)",
        &Construct::TimedInitialLiterals,
    );
}

#[test]
fn preferences() {
    let problem = problem("").replace("(:goal (p))", "(:goal (preference want (q)))");
    assert_unsupported(&domain(""), &problem, "(a)", &Construct::Preferences);
}

#[test]
fn constraints() {
    let problem = problem("(:constraints (always (p)))");
    assert_unsupported(&domain(""), &problem, "(a)", &Construct::Constraints);
}

#[test]
fn object_fluents() {
    let domain = domain("").replace(
        "(:predicates (p) (q))",
        "(:predicates (p) (q)) (:functions (holder) - object)",
    );
    assert_unsupported(&domain, &problem(""), "(a)", &Construct::ObjectFluents);
}

#[test]
fn numeric_fluents() {
    let domain = domain("")
        .replace(
            "(:predicates (p) (q))",
            "(:predicates (p) (q)) (:functions (total-cost))",
        )
        .replace(":effect (p)", ":effect (and (p) (increase (total-cost) 1))");
    assert_unsupported(&domain, &problem(""), "(a)", &Construct::NumericFluents);
}

#[test]
fn derived_predicates_through_their_own_negation() {
    let rule = "(:derived (q) (not (q)))";
    assert_unsupported(
        &domain(rule),
        &problem(""),
        "(a)",
        &Construct::UnstratifiedDerivedPredicates,
    );
}
