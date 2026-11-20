//! Level 4: PDDL+ processes and events, and every way they fail.

use std::fmt::Write;

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

/// A domain with the process `p` whose effect is `effect`.
fn process_domain(effect: &str) -> String {
    format!(
        "(define (domain d) (:requirements :time) (:predicates (on) (q)) (:functions (x))
           (:action wait :parameters ())
           (:process p :parameters () :precondition (on) :effect {effect}))"
    )
}

#[test]
fn discrete_process_effects_are_never_accepted() {
    // The parser rejects a timed effect in a process, and a `when`, whose condition
    // would be timed, even around a continuous effect.
    for effect in [
        "(at start (q))",
        "(when (at start (q)) (increase (x) (* #t 1)))",
    ] {
        let error = pddl::Domain::from_str(&process_domain(effect)).unwrap_err();
        assert!(
            error.to_string().contains("only continuous effects"),
            "{effect}: {error}"
        );
    }
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

/// Two balls falling at different speeds.
fn two_balls() -> Task {
    Task::new(
        BALL,
        "(define (problem p) (:domain ball) (:objects b1 b2 - ball)
           (:init (held b1) (held b2) (= (height b1) 10) (= (speed b1) 2)
                  (= (height b2) 10) (= (speed b2) 5) (= (bounces) 0))
           (:goal (and (landed b1) (landed b2) (= (bounces) 2))))",
    )
}

#[test]
fn events_fire_in_time_order() {
    let report = two_balls().run("0: (drop b1)\n0.01: (drop b2)\n6: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    let landings: Vec<(f64, &str)> = report
        .trace()
        .iter()
        .filter(|h| h.events.iter().any(|e| e.starts_with("event")))
        .map(|h| (h.time.unwrap(), h.events[0].as_str()))
        .collect();
    assert_eq!(landings.len(), 2, "{landings:?}");
    assert!((landings[0].0 - 2.01).abs() < 1e-9 && landings[0].1 == "event (land b2)");
    assert!((landings[1].0 - 5.0).abs() < 1e-9 && landings[1].1 == "event (land b1)");
}

#[test]
fn an_event_at_the_time_of_a_plan_step_fires_first() {
    let task = ball("(caught b1)", "");
    let kind = task.step_failure("0: (drop b1)\n5: (catch b1)", 2);
    assert!(
        matches!(
            kind,
            ViolationKind::PreconditionUnsatisfied(Unmet::Atom { .. })
        ),
        "{kind:?}"
    );
}

#[test]
fn an_undefined_fluent_in_any_grounding_is_invalid() {
    let task = Task::new(
        BALL,
        "(define (problem p) (:domain ball) (:objects b1 b2 - ball)
           (:init (held b1) (= (height b1) 10) (= (speed b1) 2) (= (bounces) 0))
           (:goal (landed b1)))",
    );
    let violation = task.violation("0: (drop b1)\n6: (wait)");
    assert!(
        matches!(violation.kind, ViolationKind::Undefined(_)),
        "{violation:?}"
    );
}

#[test]
fn plan_steps_cannot_name_processes_or_events() {
    let task = ball("(landed b1)", "");
    assert!(matches!(
        task.step_failure("0: (fall b1)", 1),
        ViolationKind::UnknownAction(_)
    ));
    assert!(matches!(
        task.step_failure("0: (land b1)", 1),
        ViolationKind::UnknownAction(_)
    ));
}

/// A process that switches on when another one's fluent crosses a bound, and an event
/// with a strict condition.
const FLOW: &str = "
(define (domain flow) (:requirements :time)
  (:predicates (on) (crossed))
  (:functions (x) (y) (z))
  (:action wait :parameters ())
  (:process drift :parameters () :precondition (on) :effect (decrease (x) #t))
  (:process fill :parameters () :effect (increase (y) #t))
  (:process spill :parameters () :precondition (> (y) 3) :effect (decrease (z) #t))
  (:event cross :parameters () :precondition (and (on) (< (x) 0))
    :effect (and (not (on)) (crossed))))";

#[test]
fn strict_events_fire_at_the_root_and_processes_switch_on() {
    let task = Task::new(
        FLOW,
        "(define (problem p) (:domain flow) (:init (on) (= (x) 2) (= (y) 0) (= (z) 10))
           (:goal (crossed)) (:metric minimize (+ (* 100 (x)) (z))))",
    );
    let report = task.run("5: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    // x stops at 0 when the event turns drifting off; spill runs from 3 to 5.
    assert!(
        (report.metric().unwrap().value - 8.0).abs() < 1e-9,
        "{report:?}"
    );
    let cross = report
        .trace()
        .iter()
        .find(|h| h.events == ["event (cross)"])
        .unwrap();
    assert!((cross.time.unwrap() - 2.0).abs() < 1e-9);
}

#[test]
fn events_at_time_zero() {
    let task = Task::new(
        TANK,
        "(define (problem p) (:domain tank) (:init (= (level) 0)) (:goal (dry)))",
    );
    let report = task.run("1: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.trace()[0].time, Some(0.0));
    assert_eq!(report.trace()[0].events, ["event (empty)"]);
}

#[test]
fn exact_comparisons_still_settle() {
    let task = Task::new(
        TANK,
        "(define (problem p) (:domain tank) (:init (= (level) 3)) (:goal (dry)))",
    );
    let report = task
        .validator()
        .with_tolerance(0.0)
        .validate_text("5: (wait)");
    // The root is not exactly 0 in f64, but the leak still settles at it.
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
}

/// A clock whose event resets it every time unit.
const CLOCK: &str = "
(define (domain clock) (:requirements :time)
  (:functions (c) (n))
  (:action wait :parameters ())
  (:process tick :parameters () :effect (increase (c) #t))
  (:event reset :parameters () :precondition (>= (c) 1)
    :effect (and (assign (c) 0) (increase (n) 1))))";

#[test]
fn periodic_events_and_the_segment_cap() {
    let task = Task::new(
        CLOCK,
        "(define (problem p) (:domain clock) (:init (= (c) 0) (= (n) 0)) (:goal (>= (n) 50))
           (:metric minimize (n)))",
    );
    let report = task.run("50.5: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert!((report.metric().unwrap().value - 50.0).abs() < 1e-9);

    // A million resets exceed the segment cap between two happenings.
    let report = task.run("1000000: (wait)");
    assert_eq!(
        report.unsupported().unwrap().construct,
        Construct::ZenoDynamics
    );
}

/// A tiny deterministic generator (64-bit LCG).
struct Lcg(u64);

impl Lcg {
    /// Returns the next pseudo-random number below `bound`.
    fn below(&mut self, bound: usize) -> usize {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        usize::try_from(self.0 >> 33).unwrap() % bound
    }
}

#[test]
fn random_hybrid_plans_never_panic() {
    const STEPS: &[&str] = &[
        "(drop b1)",
        "(drop b2)",
        "(catch b1)",
        "(catch b2)",
        "(wait)",
        "(watch b1) [5]",
        "(watch b2) [0.5]",
    ];
    const TIMES: &[&str] = &["0", "0.5", "1", "2", "2.01", "4.99", "5", "7", "1e-12"];
    let task = two_balls();
    let validator = task.validator();
    let mut random = Lcg(5);
    for _ in 0..300 {
        let mut text = String::new();
        for _ in 0..random.below(5) {
            let time = TIMES[random.below(TIMES.len())];
            let step = STEPS[random.below(STEPS.len())];
            writeln!(text, "{time}: {step}").unwrap();
        }

        let report = validator.validate_text(&text);
        assert_eq!(report, validator.validate_text(&text), "deterministic");
    }
}

/// A task in which `x` grows at rate 1 from 0, with `extra` domain items and `goal`.
fn grow(extra: &str, goal: &str) -> Task {
    Task::new(
        &format!(
            "(define (domain grow)
               (:requirements :time :durative-actions :negative-preconditions
                              :disjunctive-preconditions)
               (:predicates (done) (broken))
               (:functions (x))
               (:action wait :parameters ())
               (:process grow :parameters () :precondition (>= (x) 0)
                 :effect (increase (x) (* #t 1)))
               {extra})"
        ),
        &format!("(define (problem p) (:domain grow) (:init (= (x) 0)) (:goal {goal}))"),
    )
}

/// A durative action `hold` of duration 20 with the `over all` condition `invariant`.
fn hold(invariant: &str) -> String {
    format!(
        "(:durative-action hold :parameters () :duration (= ?duration 20)
           :condition (over all {invariant}) :effect (at end (done)))"
    )
}

#[test]
fn an_invariant_is_checked_at_an_unrelated_event() {
    let invariant = hold("(or (< (x) 10) (> (x) 10))");
    let kind = grow(&invariant, "(done)").step_failure("0: (hold) [20]", 1);
    let ViolationKind::InvariantViolated { time, .. } = kind else {
        panic!("{kind:?}");
    };
    assert!((time - 10.0).abs() < 1e-9, "{time}");

    // The event splits time at 10, where x = 10 breaks the invariant all the same.
    let mark = "(:event mark :parameters () :precondition (and (>= (x) 10) (not (done)))
                  :effect (done))";
    let task = grow(&format!("{invariant} {mark}"), "(done)");
    let kind = task.step_failure("0: (hold) [20]", 1);
    let ViolationKind::InvariantViolated { time, .. } = kind else {
        panic!("{kind:?}");
    };
    assert!((time - 10.0).abs() < 1e-9, "{time}");
}

#[test]
fn an_invariant_holds_at_the_instant_before_an_event() {
    // PDDL 2.1 and PDDL+ require an invariant at every point of the action's open
    // interval. At 10, before `reset` fires, x = 10 breaks `(< (x) 10)`.
    let reset = "(:event reset :parameters () :precondition (>= (x) 10) :effect (assign (x) 0))";
    let task = grow(&format!("{} {reset}", hold("(< (x) 10)")), "(done)");
    let kind = task.step_failure("0: (hold) [20]", 1);
    let ViolationKind::InvariantViolated { time, .. } = kind else {
        panic!("{kind:?}");
    };
    assert!((time - 10.0).abs() < 1e-9, "{time}");
}

#[test]
fn an_event_fires_before_the_events_its_effects_prevent() {
    let events = "(:event reset :parameters () :precondition (>= (x) 10) :effect (assign (x) 0))
                  (:event bad :parameters () :precondition (> (x) 10) :effect (broken))";
    let report = grow(events, "(not (broken))").run("15: (wait)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    let fired: Vec<_> = report
        .trace()
        .iter()
        .flat_map(|h| &h.events)
        .filter(|label| label.starts_with("event"))
        .collect();
    assert_eq!(fired, ["event (reset)"], "{report:?}");
}
