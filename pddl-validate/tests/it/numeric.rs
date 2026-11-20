//! Level 2: numeric conditions, updates and metrics, and every numeric way to fail.

use pddl_validate::{Atom, FluentChange, Unmet, ValidationStatus, ViolationKind};

use crate::Task;

/// A truck that burns fuel by distance, with updates of every kind.
const FUEL: &str = "
(define (domain fuel) (:requirements :typing :numeric-fluents :action-costs)
  (:types truck place)
  (:predicates (at ?t - truck ?p - place) (road ?a ?b - place))
  (:functions (fuel ?t - truck) (capacity ?t - truck) (distance ?a ?b - place)
              (load ?t - truck) (speed) (total-cost))
  (:action drive :parameters (?t - truck ?a ?b - place)
    :precondition (and (at ?t ?a) (road ?a ?b) (>= (fuel ?t) (distance ?a ?b)))
    :effect (and (not (at ?t ?a)) (at ?t ?b)
                 (decrease (fuel ?t) (distance ?a ?b))
                 (increase (total-cost) (distance ?a ?b))))
  (:action refuel :parameters (?t - truck)
    :precondition (< (fuel ?t) (capacity ?t))
    :effect (and (assign (fuel ?t) (capacity ?t)) (increase (total-cost) 1)))
  (:action double :parameters (?t - truck) :effect (scale-up (load ?t) 2))
  (:action halve :parameters (?t - truck) :effect (scale-down (load ?t) 2))
  (:action split :parameters (?t - truck) :effect (scale-down (load ?t) (speed)))
  (:action measure :parameters (?t - truck)
    :precondition (> (/ (load ?t) (speed)) 0) :effect (and))
  (:action tweak :parameters (?t - truck)
    :effect (and (assign (load ?t) 1) (increase (load ?t) 1)))
  (:action pay-twice :parameters ()
    :effect (and (increase (total-cost) 2) (decrease (total-cost) 0.5)))
  (:action grow :parameters (?t - truck) :effect (scale-up (load ?t) BIG)))";

/// A problem for [`FUEL`] with `metric` as its `:metric`.
fn problem(init: &str, metric: &str) -> String {
    format!(
        "(define (problem trip) (:domain fuel)
           (:objects t1 - truck depot shop far - place)
           (:init (at t1 depot) (road depot shop) (road shop far) (road shop depot)
                  (= (distance depot shop) 3) (= (distance shop far) 5)
                  (= (distance shop depot) 3) (= (capacity t1) 6) (= (load t1) 1)
                  (= (speed) 0) {init})
           (:goal (at t1 far))
           {metric})"
    )
}

/// The fuel task with `init` added to the initial state and `metric` as the metric.
fn task(init: &str, metric: &str) -> Task {
    let big = format!("1{}", "0".repeat(308));
    Task::new(&FUEL.replace("BIG", &big), &problem(init, metric))
}

/// The fuel task with 5 units of fuel, zero cost and a total-cost metric.
fn fuel() -> Task {
    task(
        "(= (fuel t1) 5) (= (total-cost) 0)",
        "(:metric minimize (total-cost))",
    )
}

/// The changes of the fluents a plan's step `index` (from 0) made.
fn numeric(task: &Task, plan: &str, index: usize) -> Vec<FluentChange> {
    let report = task.run(plan);
    report.trace()[index].numeric.clone()
}

/// The atom `(symbol arguments...)`.
fn fluent(symbol: &str, arguments: &[&str]) -> Atom {
    Atom::new(symbol, arguments)
}

const VALID: &str = "(drive t1 depot shop)\n(refuel t1)\n(drive t1 shop far)";

#[test]
fn valid_plan_with_metric_and_numeric_trace() {
    let task = fuel();
    let report = task.run(VALID);
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.metric().unwrap().value, 9.0);

    let changes = &report.trace()[0].numeric;
    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].fluent, fluent("fuel", &["t1"]));
    assert_eq!((changes[0].before, changes[0].after), (Some(5.0), 2.0));
    assert_eq!(changes[1].fluent, fluent("total-cost", &[]));
    assert_eq!((changes[1].before, changes[1].after), (Some(0.0), 3.0));
}

#[test]
fn unsatisfied_comparison() {
    let violation = fuel().violation("(drive t1 depot shop)\n(drive t1 shop far)");
    assert_eq!(violation.step.as_ref().unwrap().number, 2);
    let unmet = Unmet::Comparison {
        condition: "(>= (fuel t1) (distance shop far))".into(),
        left: 2.0,
        right: 5.0,
        expected: true,
    };
    assert_eq!(
        violation.kind,
        ViolationKind::PreconditionUnsatisfied(unmet)
    );
    assert!(
        violation
            .advice()
            .ends_with("(Make (>= (fuel t1) (distance shop far)) true: the sides are 2 and 5)"),
        "{}",
        violation.advice()
    );
}

#[test]
fn strict_comparison_at_the_boundary() {
    let kind = fuel().step_failure("(refuel t1)\n(refuel t1)", 2);
    let unmet = Unmet::Comparison {
        condition: "(< (fuel t1) (capacity t1))".into(),
        left: 6.0,
        right: 6.0,
        expected: true,
    };
    assert_eq!(kind, ViolationKind::PreconditionUnsatisfied(unmet));
}

#[test]
fn reading_an_undefined_fluent() {
    let task = task("", "");
    let kind = task.step_failure("(drive t1 depot shop)", 1);
    assert_eq!(kind, ViolationKind::Undefined(fluent("fuel", &["t1"])));
}

#[test]
fn undefined_fails_a_disjunction_in_either_order() {
    let domain = "(define (domain d) (:predicates (p) (done)) (:functions (f))
        (:action a :parameters () :precondition (or (p) (> (f) 0)) :effect (done))
        (:action b :parameters () :precondition (or (> (f) 0) (p)) :effect (done)))";
    let task = Task::new(
        domain,
        "(define (problem x) (:domain d) (:init (p)) (:goal (done)))",
    );
    for plan in ["(a)", "(b)"] {
        let kind = task.step_failure(plan, 1);
        assert_eq!(kind, ViolationKind::Undefined(fluent("f", &[])), "{plan}");
    }
}

#[test]
fn scaling() {
    let task = fuel();
    let changes = numeric(&task, "(double t1) (halve t1) (halve t1)", 2);
    assert_eq!((changes[0].before, changes[0].after), (Some(1.0), 0.5));
}

#[test]
fn division_by_zero() {
    let task = fuel();
    let kind = task.step_failure("(split t1)", 1);
    assert_eq!(
        kind,
        ViolationKind::DivisionByZero("(scale-down (load t1) 0)".into())
    );

    let kind = task.step_failure("(measure t1)", 1);
    assert_eq!(
        kind,
        ViolationKind::DivisionByZero("(/ (load t1) (speed))".into())
    );
}

#[test]
fn overflow() {
    let kind = fuel().step_failure("(grow t1) (grow t1)", 2);
    assert_eq!(kind, ViolationKind::NonFinite("(load t1)".into()));
}

#[test]
fn conflicting_updates() {
    let kind = fuel().step_failure("(tweak t1)", 1);
    assert_eq!(
        kind,
        ViolationKind::ConflictingUpdates(fluent("load", &["t1"]))
    );
}

#[test]
fn increases_and_decreases_of_one_fluent_add_up() {
    let changes = numeric(&fuel(), "(pay-twice)", 0);
    assert_eq!((changes[0].before, changes[0].after), (Some(0.0), 1.5));
}

#[test]
fn total_cost_starts_at_zero() {
    let task = task("(= (fuel t1) 5)", "(:metric minimize (total-cost))");
    assert_eq!(task.run(VALID).metric().unwrap().value, 9.0);
}

#[test]
fn metrics() {
    let task = task(
        "(= (fuel t1) 5)",
        "(:metric maximize (+ (* 2 (fuel t1)) (total-time)))",
    );
    let metric = *task.run(VALID).metric().unwrap();
    assert_eq!(metric.optimization, pddl::Optimization::Maximize);
    assert_eq!(metric.value, 2.0 * 1.0 + 3.0);

    let task = task_with_undefined_metric();
    let violation = task.violation(VALID);
    assert_eq!(violation.step, None);
    assert_eq!(
        violation.kind,
        ViolationKind::Undefined(fluent("load", &["shop"]))
    );
}

/// A fuel task whose metric reads `(load shop)`, which no step defines.
fn task_with_undefined_metric() -> Task {
    let domain = FUEL
        .replace("(load ?t - truck)", "(load ?t - object)")
        .replace("BIG", "2");
    Task::new(
        &domain,
        &problem("(= (fuel t1) 5)", "(:metric minimize (load shop))"),
    )
}

#[test]
fn numeric_goals() {
    let task = fuel();
    let goal = problem("(= (fuel t1) 5)", "").replace(
        "(:goal (at t1 far))",
        "(:goal (and (at t1 far) (> (fuel t1) 1)))",
    );
    let task2 = Task::new(&FUEL.replace("BIG", "2"), &goal);
    let violation = task2.violation(VALID);
    assert!(
        matches!(
            violation.kind,
            ViolationKind::GoalUnsatisfied(Unmet::Comparison { .. })
        ),
        "{violation:?}"
    );
    assert_eq!(task.run(VALID).status(), ValidationStatus::Valid);
}
