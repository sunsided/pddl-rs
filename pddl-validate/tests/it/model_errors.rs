//! Malformed domains and problems fail [`Validator::new`] before any plan runs.

use pddl::{Domain, Parser, Problem};
use pddl_validate::{Construct, ModelError, ModelErrorKind, ValidationStatus, Validator};

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

#[test]
fn repeated_variable_in_one_list() {
    let action =
        "(:action a :parameters (?x ?X - block) :precondition (on ?x ?x) :effect (clear ?x))";
    let error = model_error(&domain(action), &problem("(clear a)"));
    assert_eq!(error.location, "action `a`");
    assert_eq!(error.kind, ModelErrorKind::DuplicateVariable("x".into()));

    let body = "(:derived (clear ?x ?x - block) (on ?x table))";
    let error = model_error(
        &domain(body),
        &problem("(clear a)").replace("(:init (clear a))", "(:init)"),
    );
    assert_eq!(error.kind, ModelErrorKind::DuplicateVariable("x".into()));

    let action = "(:action a :parameters () :precondition (forall (?y ?y - block) (clear ?y))
                    :effect (clear table))";
    let error = model_error(&domain(action), &problem("(clear a)"));
    assert_eq!(error.kind, ModelErrorKind::DuplicateVariable("y".into()));
}

#[test]
fn shadowing_across_lists_is_allowed() {
    let action = "(:action a :parameters (?x - block)
                    :precondition (forall (?x - block) (clear ?x)) :effect (clear ?x))";
    let domain = Domain::from_str(&domain(action)).expect("the domain parses");
    let problem = Problem::from_str(&problem("(clear a)")).expect("the problem parses");
    Validator::new(&domain, &problem).expect("shadowing is allowed");
}

/// A domain with one predicate set by timed initial literals.
const TIMED: &str = "(define (domain t) (:requirements :timed-initial-literals)
  (:predicates (q) (done))
  (:action finish :parameters () :precondition (q) :effect (done)))";

/// A problem for [`TIMED`] with the timed literals `timed`.
fn timed_problem(timed: &str) -> String {
    format!("(define (problem p) (:domain t) (:init {timed}) (:goal (done)))")
}

#[test]
fn contradictory_timed_literals() {
    let error = model_error(TIMED, &timed_problem("(at 1 (q)) (at 1 (not (q)))"));
    assert_eq!(error.location, "init");
    assert_eq!(
        error.kind,
        ModelErrorKind::ContradictoryTimedLiterals {
            atom: "(q)".into(),
            time: "1".into()
        }
    );

    // The same literal twice, or opposite literals at different times, are fine.
    for timed in ["(at 1 (q)) (at 1 (q))", "(at 1 (q)) (at 2 (not (q)))"] {
        let domain = Domain::from_str(TIMED).expect("the domain parses");
        let problem = Problem::from_str(&timed_problem(timed)).expect("the problem parses");
        let validator = Validator::new(&domain, &problem).expect("the literals agree");
        let report = validator.validate_text("1.5: (finish)");
        assert_eq!(
            report.status(),
            ValidationStatus::Valid,
            "{timed}: {report:?}"
        );
    }
}

#[test]
fn unknown_functions_inside_numeric_expressions() {
    let domain = |body: &str| {
        format!(
            "(define (domain n) (:requirements :numeric-fluents) (:predicates (p))
               (:functions (x)) {body})"
        )
    };
    let problem = |metric: &str| {
        format!("(define (problem q) (:domain n) (:init (= (x) 0)) (:goal (p)) {metric})")
    };
    for precondition in ["(> (- (zz)) 0)", "(> (+ (x) 1 (zz)) 0)"] {
        let action = format!("(:action a :parameters () :precondition {precondition} :effect (p))");
        let error = model_error(&domain(&action), &problem(""));
        assert_eq!(error.location, "action `a`", "{precondition}");
        assert_eq!(error.kind, ModelErrorKind::UnknownFunction("zz".into()));
    }

    let error = model_error(&domain(""), &problem("(:metric minimize (- (zz)))"));
    assert_eq!(error.location, "metric");
    assert_eq!(error.kind, ModelErrorKind::UnknownFunction("zz".into()));
}

#[test]
fn processes_and_events_check_their_preconditions() {
    let domain = |items: &str| {
        format!(
            "(define (domain h) (:requirements :time) (:predicates (on)) (:functions (x)) {items})"
        )
    };
    let problem = "(define (problem q) (:domain h) (:init (= (x) 0)) (:goal (on)))";
    let error = model_error(
        &domain("(:process p :parameters () :precondition (off))"),
        problem,
    );
    assert_eq!(error.location, "process `p`");
    assert_eq!(error.kind, ModelErrorKind::UnknownPredicate("off".into()));

    let error = model_error(
        &domain("(:event e :parameters () :precondition (on) :effect (increase (zz) 1))"),
        problem,
    );
    assert_eq!(error.location, "event `e`");
    assert_eq!(error.kind, ModelErrorKind::UnknownFunction("zz".into()));
}

#[test]
fn setting_equality_in_the_initial_state() {
    let error = model_error(
        &domain(""),
        &problem("(clear a)").replace("(:init (clear a))", "(:init (= a b))"),
    );
    assert_eq!(error.location, "init");
    assert_eq!(error.kind, ModelErrorKind::EqualitySet);
}

#[test]
fn object_fluents_in_formulas_are_unsupported_not_malformed() {
    let functions = "(:functions (holder))";
    let cases = [
        (
            "(:action a :parameters () :precondition (clear (holder)) :effect (clear a))",
            "(:init (clear a))",
        ),
        (
            "(:action a :parameters () :effect (assign (holder) table))",
            "(:init (clear a))",
        ),
        ("", "(:init (clear a) (= (holder) a))"),
    ];
    for (action, init) in cases {
        let domain =
            Domain::from_str(&domain(&format!("{functions} {action}"))).expect("the domain parses");
        let problem = Problem::from_str(&problem("(clear a)").replace("(:init (clear a))", init))
            .expect("the problem parses");
        let validator = Validator::new(&domain, &problem).expect("the task is well-formed");
        let report = validator.validate_text("");
        assert_eq!(
            report.status(),
            ValidationStatus::Unsupported,
            "{action} {init}"
        );
        assert_eq!(
            report.unsupported().unwrap().construct,
            Construct::ObjectFluents
        );
    }
}

#[test]
fn a_quantified_variable_is_out_of_scope_after_its_quantifier() {
    let action = "(:action a :parameters ()
                    :precondition (and (forall (?y - block) (clear ?y)) (clear ?y))
                    :effect (clear table))";
    let error = model_error(&domain(action), &problem("(clear a)"));
    assert_eq!(error.kind, ModelErrorKind::UnboundVariable("y".into()));
}

/// A domain with the durative action `act`, whose duration, condition and effect are
/// `duration`, `condition` and `effect`, and the process `flow` with `process`.
fn durative_domain(duration: &str, condition: &str, effect: &str, process: &str) -> String {
    format!(
        "(define (domain d) (:requirements :time) (:predicates (on)) (:functions (x))
           (:durative-action act :parameters () :duration {duration}
             :condition {condition} :effect {effect})
           (:process flow :parameters () :precondition (on) :effect {process}))"
    )
}

#[test]
fn durative_actions_and_processes_check_their_names() {
    let ok = [
        "(= ?duration 1)",
        "(at start (on))",
        "(at end (on))",
        "(increase (x) (* #t 1))",
    ];
    let cases = [
        (
            0,
            "(= ?duration (y))",
            ModelErrorKind::UnknownFunction("y".into()),
        ),
        (
            1,
            "(at start (off))",
            ModelErrorKind::UnknownPredicate("off".into()),
        ),
        (
            2,
            "(at end (off))",
            ModelErrorKind::UnknownPredicate("off".into()),
        ),
        (
            2,
            "(at end (increase (x) (* ?duration (y))))",
            ModelErrorKind::UnknownFunction("y".into()),
        ),
        (
            3,
            "(increase (y) (* #t 1))",
            ModelErrorKind::UnknownFunction("y".into()),
        ),
    ];
    let problem = "(define (problem p) (:domain d) (:init (= (x) 0)) (:goal (on)))";
    for (part, text, expected) in cases {
        let mut parts = ok;
        parts[part] = text;
        let error = model_error(
            &durative_domain(parts[0], parts[1], parts[2], parts[3]),
            problem,
        );
        assert_eq!(error.kind, expected, "{text}");
    }
    let domain = Domain::from_str(&durative_domain(ok[0], ok[1], ok[2], ok[3])).unwrap();
    let problem = Problem::from_str(problem).unwrap();
    Validator::new(&domain, &problem).expect("the well-formed variant passes");
}
