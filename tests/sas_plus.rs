//! Grounds the fixtures in `tests/data/sas-plus` (the PDDL inputs of the
//! `sas-plus` translator fixtures) and checks the results against known optima.

use pddl::sas_plus::{ground, GroundError};
use pddl::{Domain, Parser, Problem};
use sas_plus::fdr::SasFile;

fn ground_fixture(name: &str) -> Result<SasFile, GroundError> {
    let dir = format!("{}/tests/data/sas-plus/{name}", env!("CARGO_MANIFEST_DIR"));
    let read = |file: &str| std::fs::read_to_string(format!("{dir}/{file}")).unwrap();
    let domain = Domain::from_str(&read("domain.pddl")).unwrap();
    let problem = Problem::from_str(&read("problem.pddl")).unwrap();
    ground(&domain, &problem)
}

/// Grounds a fixture and checks that the output parses back to the same file.
fn ground_round_trip(name: &str) -> SasFile {
    let file = ground_fixture(name).unwrap();
    let text = file.to_string();
    assert_eq!(
        text.parse::<SasFile>().unwrap(),
        file,
        "{name} does not round-trip"
    );
    file
}

/// Checks the optimal plan cost with exhaustive search.
fn assert_optimum(name: &str, optimum: u64) {
    let task = ground_round_trip(name).to_task();
    task.validate().unwrap();
    let h_star = task.perfect_heuristic();
    assert_eq!(h_star(&task.initial), Some(optimum), "{name}");
}

#[test]
fn sussman_anomaly() {
    assert_optimum("sussman-anomaly", 3);
}

#[test]
fn gripper_2() {
    assert_optimum("gripper-2", 3);
}

#[test]
fn gripper_4() {
    assert_optimum("gripper-4", 9);
}

#[test]
fn gripper_costs() {
    let file = ground_round_trip("gripper-costs");
    assert!(file.use_metric);
    assert_optimum("gripper-costs", 7);
}

#[test]
fn conditional_effects() {
    for name in ["courier", "get-paid4"] {
        let file = ground_round_trip(name);
        assert!(
            file.operators
                .iter()
                .flat_map(|op| &op.effects)
                .any(|effect| !effect.conditions.is_empty()),
            "{name} has no conditional effects"
        );
    }
}

#[test]
fn courier_costs_come_from_static_functions() {
    let file = ground_round_trip("courier");
    let costs: Vec<_> = file
        .operators
        .iter()
        .filter(|op| op.name.starts_with("drive "))
        .map(|op| op.cost)
        .collect();
    assert_ne!(costs, [] as [u32; 0]);
    assert!(costs.iter().all(|&cost| cost > 1), "{costs:?}");
}

#[test]
fn larger_problems() {
    for name in ["bw-large-a", "logistics-e"] {
        assert!(!ground_round_trip(name).operators.is_empty(), "{name}");
    }
}

/// Fast Downward's translator finds the same operators; only the variables
/// differ, because it merges mutex atoms into multi-valued variables.
#[test]
fn operator_counts_match_fast_downward() {
    for (name, operators) in [
        ("sussman-anomaly", 18),
        ("gripper-2", 18),
        ("gripper-4", 34),
        ("bw-large-a", 648),
        ("logistics-e", 756),
        ("gripper-costs", 18),
        ("courier", 18),
        ("get-paid4", 44),
    ] {
        assert_eq!(
            ground_fixture(name).unwrap().operators.len(),
            operators,
            "{name}"
        );
    }
}

#[test]
fn derived_predicates_are_unsupported() {
    assert_eq!(
        ground_fixture("wiring"),
        Err(GroundError::Unsupported("derived predicates"))
    );
}

/// Grounds a domain and problem given as text.
fn ground_str(domain: &str, problem: &str) -> Result<SasFile, GroundError> {
    let domain = Domain::from_str(domain).unwrap_or_else(|e| panic!("{domain}: {e:?}"));
    let problem = Problem::from_str(problem).unwrap_or_else(|e| panic!("{problem}: {e:?}"));
    ground(&domain, &problem)
}

/// A domain with a type `t`, the predicates `(p ?x)` and `(q)`, the function
/// `(f)` and the given extra sections.
fn domain_with(sections: &str) -> String {
    format!(
        "(define (domain d) (:types t) (:predicates (p ?x) (q)) (:functions (f) (total-cost))
           {sections})"
    )
}

/// A problem for [`domain_with`] with the objects `a` and `b` of type `t`.
fn problem_with(init: &str, goal: &str, extra: &str) -> String {
    format!(
        "(define (problem pr) (:domain d) (:objects a b - t)
           (:init {init}) (:goal {goal}) {extra})"
    )
}

/// An action `act` over `?x - t` with the given precondition and effect.
fn action(pre: &str, effect: &str) -> String {
    domain_with(&format!(
        "(:action act :parameters (?x - t) :precondition {pre} :effect {effect})"
    ))
}

/// Asserts that grounding fails on the given unsupported construct.
fn assert_unsupported(domain: &str, problem: &str, construct: &'static str) {
    assert_eq!(
        ground_str(domain, problem),
        Err(GroundError::Unsupported(construct)),
        "{domain}\n{problem}"
    );
}

#[test]
fn unsupported_sections_are_rejected() {
    let plain = problem_with("", "(p a)", "");
    let simple = action("(q)", "(p ?x)");
    for (domain, problem, construct) in [
        (
            domain_with(
                "(:durative-action dur :parameters () :duration (= ?duration 1)
                   :condition (at start (q)) :effect (at end (q)))",
            ),
            plain.clone(),
            "durative actions",
        ),
        (
            "(define (domain d) (:extends other) (:predicates (p ?x)))".to_string(),
            plain.clone(),
            "domain extension",
        ),
        (
            domain_with("(:constraints (always (q)))"),
            plain.clone(),
            "constraints",
        ),
        (
            "(define (domain d) (:types t) (:predicates (p ?x)) (:timeless (p a)))".to_string(),
            plain.clone(),
            "timeless literals",
        ),
        (
            simple.clone(),
            problem_with("", "(p a)", "(:metric maximize (total-cost))"),
            "metrics other than minimizing total-cost",
        ),
        (
            simple.clone(),
            problem_with("(at 5 (q))", "(p a)", ""),
            "timed initial literals",
        ),
        (
            simple.clone(),
            problem_with("(= (f) a)", "(p a)", ""),
            "object fluents",
        ),
    ] {
        assert_unsupported(&domain, &problem, construct);
    }
}

#[test]
fn unsupported_formulas_are_rejected() {
    let plain = problem_with("", "(p a)", "");
    let cost_problem = problem_with("", "(p a)", "(:metric minimize (total-cost))");
    for (pre, effect, problem, construct) in [
        ("(preference pref (q))", "(p ?x)", &plain, "preferences"),
        (
            "(not (and (q) (q)))",
            "(p ?x)",
            &plain,
            "not over a non-atomic formula",
        ),
        (
            "(forall (?y - t) (exists (?z - t) (p ?z)))",
            "(p ?x)",
            &plain,
            "nested exists",
        ),
        ("(or (q) (p ?x))", "(p ?x)", &plain, "or"),
        ("(imply (q) (p ?x))", "(p ?x)", &plain, "imply"),
        ("(> (f) 1)", "(p ?x)", &plain, "numeric fluents"),
        ("(p (f))", "(p ?x)", &plain, "object fluents"),
        (
            "(q)",
            "(when (q) (increase (total-cost) 1))",
            &cost_problem,
            "conditional action costs",
        ),
        (
            "(q)",
            "(increase (total-cost) (+ (f) 1))",
            &cost_problem,
            "numeric fluents",
        ),
        ("(q)", "(increase (f) 1)", &plain, "numeric fluents"),
        ("(q)", "(assign (f) a)", &plain, "object fluents"),
        ("(q)", "(= ?x ?x)", &plain, "equality in effects"),
    ] {
        assert_unsupported(&action(pre, effect), problem, construct);
    }
}

#[test]
fn undeclared_names_are_rejected() {
    let plain = problem_with("", "(p a)", "");
    assert_eq!(
        ground_str(
            &action("(q)", "(p ?x)"),
            "(define (problem pr) (:domain d) (:objects a - ghost) (:init) (:goal (p a)))"
        ),
        Err(GroundError::UnknownType("ghost".into()))
    );
    assert_eq!(
        ground_str(
            &action("(q)", "(p ?x)"),
            &problem_with("(p ghost)", "(p a)", "")
        ),
        Err(GroundError::UnknownObject("ghost".into()))
    );
    assert_eq!(
        ground_str(&action("(p ghost)", "(p ?x)"), &plain),
        Err(GroundError::UnknownObject("ghost".into()))
    );
    assert_eq!(
        ground_str(&action("(p ?y)", "(p ?x)"), &plain),
        Err(GroundError::UnknownVariable("y".into()))
    );
    assert_eq!(
        ground_str(
            &action("(q)", "(increase (total-cost) (f ?y))"),
            &problem_with("", "(p a)", "(:metric minimize (total-cost))")
        ),
        Err(GroundError::UnknownVariable("y".into()))
    );
}

#[test]
fn invalid_costs_are_rejected() {
    for (cost, init) in [("0.5", "(q)"), ("(f)", "(q)"), ("(f)", "(q) (= (f) 1.5)")] {
        assert_eq!(
            ground_str(
                &action(
                    "(q)",
                    &format!("(and (p ?x) (increase (total-cost) {cost}))")
                ),
                &problem_with(init, "(p a)", "(:metric minimize (total-cost))"),
            ),
            Err(GroundError::InvalidCost {
                action: "act a".into()
            }),
            "{cost} with {init}"
        );
    }
}

#[test]
fn costs_come_from_functions_without_arguments() {
    let file = ground_str(
        &action("(q)", "(and (p ?x) (increase (total-cost) (f)))"),
        &problem_with("(q) (= (f) 4)", "(p a)", "(:metric minimize (total-cost))"),
    )
    .unwrap();
    assert!(file.operators.iter().all(|op| op.cost == 4));
}

#[test]
fn quantified_preconditions_and_goals() {
    let domain = domain_with(
        "(:action fill :parameters (?x - t) :precondition (forall (?y - t) (not (p ?y)))
           :effect (and (p ?x) (when (forall (?y - t) (p ?y)) (q))))
         (:action pick :parameters ()
           :precondition (and (exists (?y - t) (and (p ?y) (q))))
           :effect (not (q)))",
    );
    let file = ground_str(
        &domain,
        &problem_with(
            "(q) (not (p b))",
            "(and (forall (?x - t) (p ?x)) (not (q)))",
            "",
        ),
    )
    .unwrap();
    let mut names: Vec<_> = file.operators.iter().map(|op| op.name.as_str()).collect();
    names.sort_unstable();
    // As in Fast Downward, the `exists` variable is not part of the name.
    assert_eq!(names, ["fill a", "fill b", "pick", "pick"]);
}

#[test]
fn contradictory_and_unreachable_preconditions_drop_actions() {
    let domain = "(define (domain d) (:types t) (:predicates (p ?x) (q) (r ?x))
         (:action set :parameters (?x - t) :precondition (q) :effect (p ?x))
         (:action never :parameters (?x - t) :precondition (and (p ?x) (not (p ?x)))
           :effect (not (q)))
         (:action stuck :parameters (?x - t) :precondition (r ?x) :effect (not (p ?x)))
         (:action free :parameters (?x - t) :precondition (not (r ?x)) :effect (r ?x)))";
    let file = ground_str(domain, &problem_with("(q)", "(p a)", "")).unwrap();
    assert!(file
        .operators
        .iter()
        .all(|op| !op.name.starts_with("never")));
}

#[test]
fn shared_supertypes_list_objects_once() {
    let file = ground_str(
        "(define (domain d) (:types u v - t t) (:constants a - u) (:predicates (p ?x))
           (:action set :parameters (?x - (either t u)) :precondition () :effect (p ?x)))",
        "(define (problem pr) (:domain d) (:objects a - u b - v) (:init)
           (:goal (p a)))",
    );
    let file = file.unwrap();
    assert_eq!(file.operators.len(), 2);
}

/// The atoms an operator's effects set, as `Atom r(a)` or `NegatedAtom r(a)`.
fn effect_values<'a>(file: &'a SasFile, op: &str) -> Vec<&'a str> {
    file.operators
        .iter()
        .filter(|o| o.name == op)
        .flat_map(|o| &o.effects)
        .map(|e| file.variables[e.var].values[usize::try_from(e.post).unwrap()].as_str())
        .collect()
}

#[test]
fn exists_binders_do_not_capture_parameters() {
    let file = ground_str(
        "(define (domain d) (:types t) (:predicates (p ?x) (r ?x))
           (:action act :parameters (?x - t) :precondition (exists (?x - t) (p ?x))
             :effect (r ?x)))",
        &problem_with("(p b)", "(r a)", ""),
    )
    .unwrap();
    assert_eq!(effect_values(&file, "act a"), ["Atom r(a)"]);
    assert_eq!(effect_values(&file, "act b"), ["Atom r(b)"]);
}

#[test]
fn sibling_exists_binders_are_independent() {
    let file = ground_str(
        "(define (domain d) (:types t) (:predicates (p ?x) (s ?x) (q))
           (:action act :parameters ()
             :precondition (and (exists (?y - t) (p ?y)) (exists (?y - t) (s ?y)))
             :effect (q)))",
        &problem_with("(p a) (s b)", "(q)", ""),
    )
    .unwrap();
    assert_eq!(effect_values(&file, "act"), ["Atom q()"]);
}
