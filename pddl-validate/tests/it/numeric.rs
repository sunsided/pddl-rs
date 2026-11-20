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

/// Conditions over the undefined fluent `f` and the partially defined `h`, to pin the
/// three-valued (Kleene) logic of conditions. `(p)` and `(has a)` are true, `(q)` is
/// false, and `(h ?x)` and `(ok ?x)` hold only for `a`.
const KLEENE: &str = "
(define (domain kleene)
  (:requirements :adl :derived-predicates :numeric-fluents)
  (:types t)
  (:predicates (p) (q) (done) (has ?x - t) (ok ?x - t))
  (:functions (f) (g) (h ?x - t))
  (:derived (ok ?x - t) (> (h ?x) 0))
  (:action or-pf :parameters () :precondition (or (p) (> (f) 0)) :effect (done))
  (:action or-fp :parameters () :precondition (or (> (f) 0) (p)) :effect (done))
  (:action or-qf :parameters () :precondition (or (q) (> (f) 0)) :effect (done))
  (:action and-qf :parameters () :precondition (and (q) (> (f) 0)) :effect (done))
  (:action and-fq :parameters () :precondition (and (> (f) 0) (q)) :effect (done))
  (:action not-f :parameters () :precondition (not (> (f) 0)) :effect (done))
  (:action guarded :parameters ()
    :precondition (forall (?x - t) (imply (has ?x) (> (h ?x) 0))) :effect (done))
  (:action ok-a :parameters () :precondition (ok a) :effect (done))
  (:action ok-b :parameters () :precondition (ok b) :effect (done))
  (:action read :parameters () :effect (and (done) (increase (g) (f))))
  (:action when-f :parameters () :effect (when (> (f) 0) (done))))";

/// The [`KLEENE`] task, with `metric` as its `:metric`.
fn kleene(metric: &str) -> Task {
    Task::new(
        KLEENE,
        &format!(
            "(define (problem k) (:domain kleene) (:objects a b - t)
               (:init (p) (has a) (= (h a) 1) (= (g) 0)) (:goal (done)) {metric})"
        ),
    )
}

#[test]
fn a_disjunction_holds_with_an_undefined_alternative_in_either_order() {
    let task = kleene("");
    for plan in ["(or-pf)", "(or-fp)"] {
        assert_eq!(task.run(plan).status(), ValidationStatus::Valid, "{plan}");
    }
}

#[test]
fn a_disjunction_without_a_true_alternative_is_undefined() {
    let kind = kleene("").step_failure("(or-qf)", 1);
    assert_eq!(kind, ViolationKind::Undefined(fluent("f", &[])));
}

#[test]
fn a_conjunction_blames_its_false_part_over_an_undefined_one_in_either_order() {
    let task = kleene("");
    let q = Unmet::Atom {
        atom: fluent("q", &[]),
        expected: true,
        derived: false,
    };
    for plan in ["(and-qf)", "(and-fq)"] {
        let kind = task.step_failure(plan, 1);
        assert_eq!(
            kind,
            ViolationKind::PreconditionUnsatisfied(q.clone()),
            "{plan}"
        );
    }
}

#[test]
fn the_negation_of_an_undefined_comparison_is_undefined() {
    let kind = kleene("").step_failure("(not-f)", 1);
    assert_eq!(kind, ViolationKind::Undefined(fluent("f", &[])));
}

#[test]
fn a_guarded_universal_skips_undefined_instances() {
    assert_eq!(
        kleene("").run("(guarded)").status(),
        ValidationStatus::Valid
    );
}

#[test]
fn an_undefined_rule_body_does_not_derive() {
    let task = kleene("");
    assert_eq!(task.run("(ok-a)").status(), ValidationStatus::Valid);

    let kind = task.step_failure("(ok-b)", 1);
    let unmet = Unmet::Atom {
        atom: fluent("ok", &["b"]),
        expected: true,
        derived: true,
    };
    assert_eq!(kind, ViolationKind::PreconditionUnsatisfied(unmet));
}

#[test]
fn effects_that_read_an_undefined_fluent_are_invalid() {
    let task = kleene("");
    for plan in ["(read)", "(when-f)"] {
        let kind = task.step_failure(plan, 1);
        assert_eq!(kind, ViolationKind::Undefined(fluent("f", &[])), "{plan}");
    }
}

#[test]
fn an_undefined_metric_leaves_the_plan_valid() {
    let report = kleene("(:metric minimize (f))").run("(or-pf)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.metric(), None);
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

    // An undefined metric value leaves the plan valid, without a metric.
    let report = task_with_undefined_metric().run(VALID);
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.metric(), None);
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

/// Compares two fluents with each of `<`, `=` and `>`.
const COMPARE: &str = "
(define (domain compare) (:predicates (done)) (:functions (x) (y))
  (:action lt :parameters () :precondition (< (x) (y)) :effect (done))
  (:action eq :parameters () :precondition (= (x) (y)) :effect (done))
  (:action gt :parameters () :precondition (> (x) (y)) :effect (done))
  (:action same :parameters () :precondition (and (= (x) (y)) (>= (x) (y)) (<= (x) (y))) :effect (done))
  (:action differ :parameters () :precondition (not (= (x) (y))) :effect (done)))";

#[test]
fn exactly_one_of_less_equal_greater_holds() {
    let pairs = [
        ("0", "0"),
        ("1", "2"),
        ("2", "1"),
        ("0.30000000000000004", "0.3"),
        ("0.000000000001", "0"),
        ("1000000", "1000000.0000001"),
        ("123456789", "123456789.5"),
    ];
    for (x, y) in pairs {
        let problem = format!(
            "(define (problem p) (:domain compare) (:init (= (x) {x}) (= (y) {y})) (:goal (done)))"
        );
        let task = Task::new(COMPARE, &problem);
        for tolerance in [pddl_validate::DEFAULT_TOLERANCE, 0.0] {
            let validator = task.validator().with_tolerance(tolerance);
            let holds =
                |plan: &str| validator.validate_text(plan).status() == ValidationStatus::Valid;
            let count = ["(lt)", "(eq)", "(gt)"]
                .into_iter()
                .filter(|plan| holds(plan))
                .count();
            assert_eq!(count, 1, "{x} vs {y} at {tolerance}");
            assert_eq!(holds("(eq)"), holds("(same)"), "{x} vs {y} at {tolerance}");
            assert_ne!(
                holds("(eq)"),
                holds("(differ)"),
                "{x} vs {y} at {tolerance}"
            );
        }
    }
}

#[test]
fn negated_comparison() {
    let problem =
        "(define (problem p) (:domain compare) (:init (= (x) 1) (= (y) 1)) (:goal (done)))";
    let kind = Task::new(COMPARE, problem).step_failure("(differ)", 1);
    let unmet = Unmet::Comparison {
        condition: "(= (x) (y))".into(),
        left: 1.0,
        right: 1.0,
        expected: false,
    };
    assert_eq!(unmet.to_string(), "(not (= (x) (y)))");
    assert_eq!(kind, ViolationKind::PreconditionUnsatisfied(unmet));
}

#[test]
fn equality_of_two_fluents_is_numeric() {
    let domain = "(define (domain d) (:predicates (done) (marked)) (:functions (f ?o) (g ?o))
        (:action same :parameters (?o) :precondition (= (f ?o) (g ?o))
          :effect (and (done) (when (not (= (f ?o) (g ?o))) (marked))))
        (:action bump :parameters (?o) :effect (increase (f ?o) 1)))";
    let problem = "(define (problem p) (:domain d) (:objects a) (:init (= (f a) 2) (= (g a) 2))
        (:goal (and (done) (not (marked)) (= (f a) (g a)))))";
    let task = Task::new(domain, problem);
    assert_eq!(task.run("(same a)").status(), ValidationStatus::Valid);

    let kind = task.step_failure("(bump a) (same a)", 2);
    let unmet = Unmet::Comparison {
        condition: "(= (f a) (g a))".into(),
        left: 3.0,
        right: 2.0,
        expected: true,
    };
    assert_eq!(kind, ViolationKind::PreconditionUnsatisfied(unmet));
}

#[test]
fn comparisons_in_derived_predicates() {
    let domain = "(define (domain d) (:requirements :derived-predicates :numeric-fluents)
        (:predicates (full) (done)) (:functions (fuel))
        (:derived (full) (>= (fuel) 3))
        (:action fill :parameters () :effect (increase (fuel) 1))
        (:action go :parameters () :precondition (full) :effect (done)))";
    let problem = "(define (problem p) (:domain d) (:init (= (fuel) 1)) (:goal (done)))";
    let task = Task::new(domain, problem);
    assert_eq!(
        task.run("(fill) (fill) (go)").status(),
        ValidationStatus::Valid
    );
    assert_eq!(
        task.step_failure("(fill) (go)", 2),
        ViolationKind::PreconditionUnsatisfied(Unmet::Atom {
            atom: fluent("full", &[]),
            expected: true,
            derived: true
        })
    );
}

/// A task with one fluent `(x)` set to `x`, the goal `goal`, and actions `at-most`
/// (needs `(<= (x) 1000000000)`) and `above` (needs `(> (x) 1000000000)`).
fn threshold(x: &str, goal: &str) -> Task {
    let domain = "(define (domain d) (:requirements :numeric-fluents)
        (:predicates (done)) (:functions (x))
        (:action at-most :parameters () :precondition (<= (x) 1000000000) :effect (done))
        (:action above :parameters () :precondition (> (x) 1000000000) :effect (done)))";
    let problem = format!("(define (problem p) (:domain d) (:init (= (x) {x})) (:goal {goal}))");
    Task::new(domain, &problem)
}

#[test]
fn default_tolerance_keeps_large_integers_exact() {
    let task = threshold("1000000001", "(done)");
    let kind = task.step_failure("(at-most)", 1);
    assert!(
        matches!(kind, ViolationKind::PreconditionUnsatisfied(_)),
        "{kind:?}"
    );
    assert_eq!(task.run("(above)").status(), ValidationStatus::Valid);
}

#[test]
fn default_tolerance_keeps_small_differences_near_zero() {
    let task = threshold("0.0000000005", "(<= (x) 0)");
    assert_eq!(task.run("").status(), ValidationStatus::Invalid);
}

/// The violation of `(> exp 1000)` with `(x)` = 6, `(y)` = 2 and `(z)` = 0.
fn probe(exp: &str) -> ViolationKind {
    let domain = format!(
        "(define (domain d) (:requirements :numeric-fluents) (:predicates (done))
           (:functions (x) (y) (z))
           (:action probe :parameters () :precondition (> {exp} 1000) :effect (done)))"
    );
    let problem =
        "(define (problem p) (:domain d) (:init (= (x) 6) (= (y) 2) (= (z) 0)) (:goal (done)))";
    Task::new(&domain, problem).step_failure("(probe)", 1)
}

/// The left side that [`probe`] reports for `exp`.
fn left_side(exp: &str) -> f64 {
    match probe(exp) {
        ViolationKind::PreconditionUnsatisfied(Unmet::Comparison { left, .. }) => left,
        kind => panic!("{exp}: {kind:?}"),
    }
}

#[test]
fn every_arithmetic_operator_in_conditions() {
    let cases = [
        ("(- (x))", -6.0),
        ("(+ (x) (y))", 8.0),
        ("(- (x) (y))", 4.0),
        ("(* (x) (y))", 12.0),
        ("(/ (x) (y))", 3.0),
        ("(+ (x) (y) (y))", 10.0),
        ("(* (x) (y) (y))", 24.0),
        ("(- (* (x) (y) (y)) (+ 1 (x) (- (y))))", 19.0),
    ];
    for (exp, value) in cases {
        assert_eq!(left_side(exp), value, "{exp}");
    }
}

#[test]
fn arithmetic_without_a_finite_value_leaves_a_condition_undefined() {
    let big = format!("1{}", "0".repeat(308));
    assert_eq!(
        probe(&format!("(* (x) {big} (y))")),
        ViolationKind::NonFinite(format!("(* (x) {big} (y))"))
    );
    assert_eq!(
        probe("(+ (x) (/ (y) (z)))"),
        ViolationKind::DivisionByZero("(/ (y) (z))".into())
    );
}

#[test]
fn every_arithmetic_operator_in_metrics() {
    // The valid plan costs 9 and takes 3 steps.
    let cases = [
        ("(- (total-cost))", -9.0),
        ("(- total-time)", -3.0),
        ("(/ (total-cost) 2)", 4.5),
        ("(- (total-cost) total-time)", 6.0),
        ("(* 2 (total-cost) total-time)", 54.0),
        ("(+ (total-cost) total-time 1)", 13.0),
    ];
    for (exp, value) in cases {
        let task = task(
            "(= (fuel t1) 5) (= (total-cost) 0)",
            &format!("(:metric minimize {exp})"),
        );
        let report = task.run(VALID);
        assert_eq!(report.metric().map(|m| m.value), Some(value), "{exp}");
    }
}

#[test]
fn a_metric_that_divides_by_zero_leaves_the_plan_valid_without_a_value() {
    let task = task(
        "(= (fuel t1) 5) (= (total-cost) 0)",
        "(:metric minimize (/ (total-cost) (speed)))",
    );
    let report = task.run(VALID);
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.metric(), None);
}

/// A tank that fills at one unit per time unit once opened, until the event `full`
/// fires on `CONDITION`.
const TANK: &str = "
(define (domain tank) (:requirements :time :numeric-fluents)
  (:predicates (filling) (full))
  (:functions (level))
  (:action open :parameters () :effect (filling))
  (:action wait :parameters ())
  (:process fill :parameters () :precondition (filling)
    :effect (increase (level) (* #t 1)))
  (:event stop :parameters () :precondition (and (filling) CONDITION)
    :effect (and (not (filling)) (full))))";

/// Runs the tank with the event condition `condition` and returns the report.
fn tank(condition: &str) -> pddl_validate::Report {
    let task = Task::new(
        &TANK.replace("CONDITION", condition),
        "(define (problem p) (:domain tank) (:init (= (level) 0)) (:goal (full)))",
    );
    task.run("0: (open)\n10: (wait)")
}

#[test]
fn events_on_linear_expressions_fire_when_the_level_reaches_four() {
    let conditions = [
        "(<= (- (level)) (- 4))",
        "(>= (+ (level) 1) 5)",
        "(>= (- (level) 1) 3)",
        "(>= (/ (level) 2) 2)",
        "(>= (+ (level) 1 1) 6)",
        "(>= (* 2 (level) 1) 8)",
    ];
    for condition in conditions {
        let report = tank(condition);
        assert_eq!(
            report.status(),
            ValidationStatus::Valid,
            "{condition}: {report:?}"
        );
        let stop = report
            .trace()
            .iter()
            .find(|h| h.events == ["event (stop)"])
            .expect("the event fires");
        assert!(
            (stop.time.unwrap() - 4.0).abs() < 1e-6,
            "{condition}: {stop:?}"
        );
    }
}

#[test]
fn events_on_nonlinear_expressions_are_unsupported() {
    for condition in [
        "(<= (/ 8 (+ (level) 1)) 2)",
        "(>= (* (level) (level) 1) 16)",
    ] {
        let report = tank(condition);
        assert_eq!(
            report.unsupported().map(|u| u.construct.clone()),
            Some(pddl_validate::Construct::NonlinearDynamics),
            "{condition}: {report:?}"
        );
    }
}

/// Negated connectives over the undefined `(f)`, the true `(p)` and the false `(q)`.
fn negated_kleene(precondition: &str) -> Task {
    let domain = format!(
        "(define (domain d) (:requirements :adl :numeric-fluents)
           (:types t) (:predicates (p) (q) (done)) (:functions (f) (h ?x - t))
           (:action go :parameters () :precondition {precondition} :effect (done)))"
    );
    Task::new(
        &domain,
        "(define (problem k) (:domain d) (:objects a b - t)
           (:init (p) (= (h a) 1)) (:goal (done)))",
    )
}

#[test]
fn negated_connectives_follow_kleene_logic() {
    let undefined = |name: &str| ViolationKind::Undefined(fluent(name, &[]));
    let h_b = ViolationKind::Undefined(fluent("h", &["b"]));
    let atom = |name: &str, expected: bool| {
        ViolationKind::PreconditionUnsatisfied(Unmet::Atom {
            atom: fluent(name, &[]),
            expected,
            derived: false,
        })
    };
    let h_a = ViolationKind::PreconditionUnsatisfied(Unmet::Comparison {
        condition: "(> (h a) 0)".into(),
        left: 1.0,
        right: 0.0,
        expected: false,
    });
    let failures = [
        ("(not (and (p) (> (f) 0)))", undefined("f")),
        ("(not (or (p) (> (f) 0)))", atom("p", false)),
        ("(not (or (q) (> (f) 0)))", undefined("f")),
        ("(not (imply (p) (> (f) 0)))", undefined("f")),
        ("(not (imply (q) (> (f) 0)))", atom("q", true)),
        ("(not (exists (?x - t) (> (h ?x) 0)))", h_a),
        ("(or (q) (forall (?x - t) (> (h ?x) 0)))", h_b),
    ];
    for (precondition, expected) in failures {
        let kind = negated_kleene(precondition).step_failure("(go)", 1);
        assert_eq!(kind, expected, "{precondition}");
    }

    let report = negated_kleene("(not (and (q) (> (f) 0)))").run("(go)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
}
