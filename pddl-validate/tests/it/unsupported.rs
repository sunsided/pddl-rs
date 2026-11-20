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
fn derived_predicates_in_temporal_plans() {
    let rule = "(:derived (q) (p))";
    assert_unsupported(
        &domain(rule),
        &problem(""),
        "0: (a)",
        &Construct::DerivedPredicatesInTemporalPlans,
    );
}

#[test]
fn end_conditions_of_start_effects() {
    let action = "(:durative-action go :parameters () :duration (= ?duration 1)
                    :condition (at start (q)) :effect (when (at end (q)) (at start (p))))";
    assert_unsupported(
        &domain(action),
        &problem(""),
        "0: (go) [1]",
        &Construct::TimedEffectConditions,
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
fn is_violated_in_a_metric() {
    let problem = problem("(:metric minimize (is-violated want))");
    assert_unsupported(&domain(""), &problem, "(a)", &Construct::Preferences);
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

#[test]
fn unknown_requirements() {
    let domain = domain("").replace(":strips", ":strips :teleportation");
    let report = Task::new(&domain, &problem("")).run("(a)");
    assert_eq!(
        report.unsupported().unwrap().construct,
        Construct::UnknownRequirement(":teleportation".into())
    );
}

#[test]
fn pddl_1_2_sections() {
    let extended = domain("").replace("(:requirements", "(:extends base) (:requirements");
    assert_unsupported(&extended, &problem(""), "(a)", &Construct::DomainExtension);

    let timeless = domain("").replace("(:action", "(:timeless (q)) (:action");
    assert_unsupported(&timeless, &problem(""), "(a)", &Construct::Timeless);
}

#[test]
fn over_all_conditions_of_conditional_effects() {
    let action = "(:durative-action go :parameters () :duration (= ?duration 1)
                    :effect (when (over all (q)) (at end (p))))";
    assert_unsupported(
        &domain(action),
        &problem(""),
        "0: (go) [1]",
        &Construct::TimedEffectConditions,
    );
}
