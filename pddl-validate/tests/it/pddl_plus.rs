//! Level 4: PDDL+ processes and events, and every way they fail.

use pddl::Parser;
use pddl_validate::{Construct, Unmet, ValidationStatus, ViolationKind};

use crate::Task;

/// A ball that falls at a constant speed once dropped, lands by an event, and can be
/// caught low enough or watched while it is high.
const BALL: &str = "
(define (domain ball) (:requirements :typing :time :durative-actions)
  (:types ball)
  (:predicates (held ?b - ball) (falling ?b - ball) (landed ?b - ball) (caught ?b - ball)
               (watched ?b - ball))
  (:functions (height ?b - ball) (speed ?b - ball) (bounces))
  (:action drop :parameters (?b - ball) :precondition (held ?b)
    :effect (and (not (held ?b)) (falling ?b)))
  (:action catch :parameters (?b - ball)
    :precondition (and (falling ?b) (<= (height ?b) 5))
    :effect (and (not (falling ?b)) (caught ?b)))
  (:action wait :parameters ())
  (:durative-action watch :parameters (?b - ball) :duration (= ?duration 5)
    :condition (over all (>= (height ?b) 2))
    :effect (at end (watched ?b)))
  (:process fall :parameters (?b - ball) :precondition (falling ?b)
    :effect (decrease (height ?b) (* #t (speed ?b))))
  (:event land :parameters (?b - ball)
    :precondition (and (falling ?b) (<= (height ?b) 0))
    :effect (and (not (falling ?b)) (landed ?b) (increase (bounces) 1))))";

/// A problem for [`BALL`] with `goal` and `metric`.
fn ball(goal: &str, metric: &str) -> Task {
    Task::new(
        BALL,
        &format!(
            "(define (problem p) (:domain ball) (:objects b1 - ball)
               (:init (held b1) (= (height b1) 10) (= (speed b1) 2) (= (bounces) 0))
               (:goal {goal}) {metric})"
        ),
    )
}

#[test]
fn an_event_fires_when_the_process_reaches_it() {
    let task = ball(
        "(and (landed b1) (= (bounces) 1))",
        "(:metric minimize (height b1))",
    );
    let report = task.run("0: (drop b1)\n6: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert!(report.metric().unwrap().value.abs() < 1e-9);

    let landing = report
        .trace()
        .iter()
        .find(|h| h.events == ["event (land b1)"])
        .expect("the ball lands");
    assert!((landing.time.unwrap() - 5.0).abs() < 1e-9, "{landing:?}");
}

#[test]
fn an_action_stops_a_process() {
    let task = ball("(caught b1)", "(:metric minimize (height b1))");
    let report = task.run("0: (drop b1)\n3: (catch b1)\n6: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert!((report.metric().unwrap().value - 4.0).abs() < 1e-9);

    // At time 1 the ball is still at 8.
    let kind = task.step_failure("0: (drop b1)\n1: (catch b1)", 2);
    let ViolationKind::PreconditionUnsatisfied(Unmet::Comparison { left, .. }) = kind else {
        panic!("{kind:?}");
    };
    assert!((left - 8.0).abs() < 1e-9);

    // After landing at 5 the ball no longer falls.
    let kind = task.step_failure("0: (drop b1)\n6: (catch b1)", 2);
    assert!(
        matches!(
            kind,
            ViolationKind::PreconditionUnsatisfied(Unmet::Atom { .. })
        ),
        "{kind:?}"
    );
}

#[test]
fn a_process_breaks_an_invariant() {
    let task = ball("(watched b1)", "");
    let kind = task.step_failure("0: (drop b1)\n0.5: (watch b1) [5]", 2);
    let ViolationKind::InvariantViolated { time, .. } = kind else {
        panic!("{kind:?}");
    };
    assert!(time > 4.0 && time < 5.5, "{time}");

    assert_eq!(
        task.run("0: (watch b1) [5]\n5.5: (drop b1)").status(),
        ValidationStatus::Valid
    );
}

/// A tank that leaks while it holds water; an event marks it dry.
const TANK: &str = "
(define (domain tank) (:requirements :time)
  (:predicates (dry))
  (:functions (level))
  (:action wait :parameters ())
  (:process leak :parameters () :precondition (> (level) 0) :effect (decrease (level) #t))
  (:event empty :parameters () :precondition (and (<= (level) 0) (not (dry))) :effect (dry)))";

#[test]
fn a_process_switches_off_at_its_root() {
    let task = Task::new(
        TANK,
        "(define (problem p) (:domain tank) (:init (= (level) 3)) (:goal (dry))
           (:metric minimize (level)))",
    );
    let report = task.run("5: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    // The leak stops at 0 instead of draining below.
    assert!(report.metric().unwrap().value.abs() < 1e-9);

    let violation = task.violation("2: (wait)");
    assert!(
        matches!(violation.kind, ViolationKind::GoalUnsatisfied(_)),
        "{violation:?}"
    );
}

/// Events that trigger each other, and one that keeps triggering itself.
const CHAIN: &str = "
(define (domain chain) (:requirements :time)
  (:predicates (p) (q) (r) (on))
  (:functions (n))
  (:action set :parameters () :effect (p))
  (:action switch :parameters () :effect (on))
  (:event first :parameters () :precondition (p) :effect (and (not (p)) (q)))
  (:event second :parameters () :precondition (q) :effect (and (not (q)) (r)))
  (:event runaway :parameters () :precondition (on) :effect (increase (n) 1)))";

#[test]
fn events_cascade() {
    let task = Task::new(
        CHAIN,
        "(define (problem p) (:domain chain) (:init (= (n) 0)) (:goal (r)))",
    );
    let report = task.run("1: (set)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    let events: Vec<&[String]> = report.trace().iter().map(|h| h.events.as_slice()).collect();
    assert_eq!(
        events,
        [
            &["(set)".to_string()][..],
            &["event (first)".to_string()],
            &["event (second)".to_string()]
        ]
    );

    let violation = task.violation("1: (switch)");
    assert_eq!(violation.step, None);
    assert!(
        matches!(violation.kind, ViolationKind::EventCascade { time, .. } if (time - 1.0).abs() < 1e-9),
        "{violation:?}"
    );
}

#[test]
fn nonlinear_processes_are_unsupported() {
    let accelerating = BALL.replace(
        "(decrease (height ?b) (* #t (speed ?b)))",
        "(and (decrease (height ?b) (* #t (speed ?b))) (increase (speed ?b) (* #t 9.8)))",
    );
    let task = Task::new(
        &accelerating,
        "(define (problem p) (:domain ball) (:objects b1 - ball)
           (:init (held b1) (= (height b1) 10) (= (speed b1) 0) (= (bounces) 0))
           (:goal (landed b1)))",
    );
    let report = task.run("0: (drop b1)\n6: (wait)");
    assert_eq!(
        report.unsupported().unwrap().construct,
        Construct::NonlinearDynamics
    );
}

#[test]
fn discrete_process_effects_are_unsupported() {
    let domain = "(define (domain d) (:requirements :time) (:predicates (on) (q))
        (:action wait :parameters ())
        (:process p :parameters () :precondition (on) :effect (at start (q))))";
    let report = Task::new(domain, "(define (problem x) (:domain d) (:goal (q)))").run("1: (wait)");
    assert_eq!(
        report.unsupported().unwrap().construct,
        Construct::DiscreteProcessEffects
    );
}

#[test]
fn processes_and_events_check_their_names() {
    let domain = "(define (domain d) (:requirements :time) (:predicates (on))
        (:event e :parameters () :precondition (off) :effect (on)))";
    let domain = pddl::Domain::from_str(domain).unwrap();
    let problem = pddl::Problem::from_str("(define (problem x) (:domain d) (:goal (on)))").unwrap();
    let error = pddl_validate::Validator::new(&domain, &problem).unwrap_err();
    assert_eq!(error.location, "event `e`");
}
