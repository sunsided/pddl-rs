//! Level 3: temporal plans, and every temporal way to fail.

use std::fmt::Write;

use pddl_validate::{Construct, Unmet, ValidationStatus, ViolationKind};

use crate::Task;

/// A rover that drives on a draining battery and samples in daylight.
const ROVER: &str = "
(define (domain rover)
  (:requirements :typing :durative-actions :duration-inequalities :numeric-fluents
                 :continuous-effects :timed-initial-literals)
  (:types rover place)
  (:predicates (at ?r - rover ?p - place) (road ?a ?b - place) (busy ?r - rover)
               (daylight) (calm) (sampled ?p - place))
  (:functions (battery ?r - rover) (distance ?a ?b - place) (speed) (samples))
  (:durative-action drive
    :parameters (?r - rover ?a ?b - place)
    :duration (= ?duration (/ (distance ?a ?b) (speed)))
    :condition (and (at start (at ?r ?a)) (at start (road ?a ?b)) (at start (not (busy ?r)))
                    (over all (>= (battery ?r) 0)))
    :effect (and (at start (not (at ?r ?a))) (at start (busy ?r))
                 (at end (at ?r ?b)) (at end (not (busy ?r)))
                 (decrease (battery ?r) (* 2 #t))))
  (:durative-action sample
    :parameters (?r - rover ?p - place)
    :duration (and (>= ?duration 1) (<= ?duration 5))
    :condition (and (at start (at ?r ?p)) (over all (at ?r ?p)) (over all (daylight))
                    (at end (calm)))
    :effect (and (at end (sampled ?p)) (at end (increase (samples) ?duration))))
  (:action charge :parameters (?r - rover)
    :precondition (not (busy ?r))
    :effect (assign (battery ?r) 10)))";

/// A problem for [`ROVER`] with `init` added and `metric` as the metric.
fn problem(init: &str, metric: &str) -> String {
    format!(
        "(define (problem trip) (:domain rover)
           (:objects r1 - rover base hill ridge - place)
           (:init (at r1 base) (road base hill) (road hill ridge)
                  (= (distance base hill) 4) (= (distance hill ridge) 6) (= (speed) 2)
                  (= (samples) 0) (daylight) (calm) (at 20 (not (daylight))) {init})
           (:goal (and (sampled hill) (at r1 ridge)))
           {metric})"
    )
}

/// The rover with a full battery.
fn rover() -> Task {
    Task::new(ROVER, &problem("(= (battery r1) 10)", ""))
}

/// A valid plan: drive, sample, drive on, each 0.01 after the previous end.
const VALID: &str = "
0.000: (drive r1 base hill) [2.000]
2.010: (sample r1 hill) [3.000]
5.020: (drive r1 hill ridge) [3.000]";

#[test]
fn valid_plan_with_makespan_and_continuous_change() {
    let task = Task::new(
        ROVER,
        &problem(
            "(= (battery r1) 10)",
            "(:metric minimize (+ (* 100 (total-time)) (battery r1) (samples)))",
        ),
    );
    let report = task.run(VALID);
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    // Makespan 8.02, battery 10 - 2 * (2 + 3) = 0, samples 3.
    let value = report.metric().unwrap().value;
    assert!((value - (802.0 + 0.0 + 3.0)).abs() < 1e-9, "{value}");

    let happenings: Vec<Option<f64>> = report.trace().iter().map(|h| h.time).collect();
    assert_eq!(
        happenings,
        [
            Some(0.0),
            Some(2.0),
            Some(2.01),
            Some(5.01),
            Some(5.02),
            Some(8.02)
        ]
    );
    assert_eq!(report.trace()[0].events, ["start (drive r1 base hill)"]);
}

#[test]
fn untimed_plans_run_at_integer_times() {
    let task = Task::new(
        ROVER,
        &problem("(= (battery r1) 10)", "(:metric minimize (total-time))"),
    );
    let kind = task.step_failure("(charge r1)\n(drive r1 base hill)", 2);
    assert_eq!(kind, ViolationKind::MissingDuration);
}

#[test]
fn missing_and_unexpected_durations() {
    let task = rover();
    assert_eq!(
        task.step_failure("0: (drive r1 base hill)", 1),
        ViolationKind::MissingDuration
    );
    assert_eq!(
        task.step_failure("0: (charge r1) [1]", 1),
        ViolationKind::UnexpectedDuration
    );
    assert_eq!(
        task.step_failure("0: (drive r1 base hill) [0]", 1),
        ViolationKind::NonPositiveDuration(0.0)
    );
    assert_eq!(
        task.step_failure("0: (charge r1)\n(charge r1)", 2),
        ViolationKind::MissingTimestamp
    );
}

#[test]
fn duration_constraints() {
    let task = rover();
    let kind = task.step_failure("0: (drive r1 base hill) [3]", 1);
    assert_eq!(
        kind,
        ViolationKind::DurationConstraint(Unmet::Comparison {
            condition: "(= ?duration (/ (distance base hill) (speed)))".into(),
            left: 3.0,
            right: 2.0,
            expected: true
        })
    );

    let kind = task.step_failure("0: (drive r1 base hill) [2]\n2.01: (sample r1 hill) [6]", 2);
    assert!(
        matches!(&kind, ViolationKind::DurationConstraint(Unmet::Comparison { condition, .. }) if condition == "(<= ?duration 5)"),
        "{kind:?}"
    );
}

#[test]
fn start_and_end_conditions() {
    let task = rover();
    let kind = task.step_failure("0: (sample r1 hill) [2]", 1);
    assert!(
        matches!(
            kind,
            ViolationKind::StartConditionUnsatisfied(Unmet::Atom { .. })
        ),
        "{kind:?}"
    );

    // A timed initial literal ends the calm before the sample ends.
    let task = Task::new(
        ROVER,
        &problem("(= (battery r1) 10) (at 4 (not (calm)))", ""),
    );
    let kind = task.step_failure(VALID, 2);
    assert!(
        matches!(
            kind,
            ViolationKind::EndConditionUnsatisfied(Unmet::Atom { .. })
        ),
        "{kind:?}"
    );
}

#[test]
fn invariant_broken_by_a_timed_initial_literal() {
    let plan = "0: (drive r1 base hill) [2]\n18: (sample r1 hill) [3]";
    let kind = rover().step_failure(plan, 2);
    let ViolationKind::InvariantViolated { time, unmet } = kind else {
        panic!("{kind:?}");
    };
    assert!((20.0..21.0).contains(&time), "{time}");
    assert_eq!(unmet.to_string(), "(daylight)");
}

#[test]
fn invariant_broken_by_another_action() {
    let plan =
        "0: (drive r1 base hill) [2]\n2.01: (sample r1 hill) [3]\n3: (drive r1 hill ridge) [3]";
    let kind = rover().step_failure(plan, 2);
    let ViolationKind::InvariantViolated { time, unmet } = kind else {
        panic!("{kind:?}");
    };
    assert!((3.0..5.01).contains(&time), "{time}");
    assert_eq!(unmet.to_string(), "(at r1 hill)");
}

#[test]
fn invariant_broken_by_continuous_change() {
    // With 5 units, the second drive empties the battery half a time unit in.
    let task = Task::new(ROVER, &problem("(= (battery r1) 5)", ""));
    let kind = task.step_failure(VALID, 3);
    let ViolationKind::InvariantViolated { time, unmet } = kind else {
        panic!("{kind:?}");
    };
    assert!(time > 5.52 && time < 8.02, "{time}");
    let Unmet::Comparison { left, .. } = unmet else {
        panic!("{unmet:?}");
    };
    assert!(left < 0.0);

    // With exactly enough charge the battery reaches 0 at the end, which `>=` allows.
    let task = Task::new(ROVER, &problem("(= (battery r1) 10)", ""));
    assert_eq!(task.run(VALID).status(), ValidationStatus::Valid);
}

#[test]
fn interference() {
    let task = rover();
    let simultaneous =
        "0: (drive r1 base hill) [2]\n2: (sample r1 hill) [3]\n5.02: (drive r1 hill ridge) [3]";
    let kind = task.step_failure(simultaneous, 2);
    assert_eq!(
        kind,
        ViolationKind::Interference {
            event: "start (sample r1 hill)".into(),
            other: "end (drive r1 base hill)".into(),
            time: 2.0,
            separation: 0.0
        }
    );

    let close =
        "0: (drive r1 base hill) [2]\n2.005: (sample r1 hill) [3]\n5.02: (drive r1 hill ridge) [3]";
    let kind = task.step_failure(close, 2);
    assert!(
        matches!(kind, ViolationKind::Interference { .. }),
        "{kind:?}"
    );

    let report = task.validator().with_epsilon(0.001).validate_text(close);
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
}

#[test]
fn nonlinear_dynamics_are_unsupported() {
    let feedback = ROVER.replace("(* 2 #t)", "(* (battery ?r) #t)");
    let report = Task::new(&feedback, &problem("(= (battery r1) 10)", "")).run(VALID);
    assert_eq!(
        report.unsupported().unwrap().construct,
        Construct::NonlinearDynamics
    );

    let square = ROVER.replace(
        "(over all (>= (battery ?r) 0))",
        "(over all (>= (* (battery ?r) (battery ?r)) 0))",
    );
    let report = Task::new(&square, &problem("(= (battery r1) 10)", "")).run(VALID);
    assert_eq!(
        report.unsupported().unwrap().construct,
        Construct::NonlinearDynamics
    );
}

/// A lamp whose `at end` effect depends on an `at start` condition.
const LAMP: &str = "
(define (domain lamp) (:requirements :durative-actions :conditional-effects)
  (:predicates (on) (bright) (done))
  (:durative-action shine :parameters () :duration (= ?duration 2)
    :condition (at start (not (done)))
    :effect (and (at start (done)) (when (at start (on)) (at end (bright)))))
  (:action toggle :parameters ()
    :effect (and (when (on) (not (on))) (when (not (on)) (on)))))";

#[test]
fn start_conditions_of_end_effects_are_remembered() {
    let task = Task::new(
        LAMP,
        "(define (problem p) (:domain lamp) (:init (on)) (:goal (bright)))",
    );
    assert_eq!(task.run("0: (shine) [2]").status(), ValidationStatus::Valid);
    assert_eq!(
        task.run("0: (shine) [2]\n1: (toggle)").status(),
        ValidationStatus::Valid
    );

    let violation = task.violation("0: (toggle)\n1: (shine) [2]");
    assert!(
        matches!(violation.kind, ViolationKind::GoalUnsatisfied(_)),
        "{violation:?}"
    );
}

#[test]
fn timed_literals_interfere_like_events() {
    // The sample reads (calm) at its end, when a timed literal deletes it.
    let task = Task::new(
        ROVER,
        &problem("(= (battery r1) 10) (at 4 (not (calm)))", ""),
    );
    let violation = task.violation("0: (drive r1 base hill) [2]\n2.01: (sample r1 hill) [1.99]");
    assert_eq!(violation.step, None);
    let ViolationKind::Interference { event, other, .. } = violation.kind else {
        panic!("{violation:?}");
    };
    assert_eq!(event, "timed literal (not (calm)) at 4");
    assert_eq!(other, "end (sample r1 hill)");

    // An over all condition the literal breaks later is an invariant violation instead.
    let kind = rover().step_failure("0: (drive r1 base hill) [2]\n20: (sample r1 hill) [3]", 2);
    assert!(
        matches!(kind, ViolationKind::InvariantViolated { .. }),
        "{kind:?}"
    );
}

#[test]
fn mutations_of_a_temporal_plan() {
    let task = rover();
    let validator = task.validator();
    let plan: pddl_validate::Plan = VALID.parse().unwrap();
    let steps = plan.steps();
    for skip in 0..steps.len() {
        let mut dropped = steps.to_vec();
        dropped.remove(skip);
        let report = validator.validate(&pddl_validate::Plan::new(dropped));
        assert_eq!(
            report.status(),
            ValidationStatus::Invalid,
            "dropping {skip}"
        );
    }

    let mut invalid = 0;
    for index in 0..steps.len() {
        for shift in [-0.5, -0.005, 0.005, 0.5] {
            let mut shifted = steps.to_vec();
            let time = shifted[index].time().unwrap() + shift;
            let duration = shifted[index].duration();
            let mut step = shifted[index].clone().at(time.max(0.0));
            if let Some(duration) = duration {
                step = step.lasting(duration);
            }
            shifted[index] = step;
            let plan = pddl_validate::Plan::new(shifted);
            let report = validator.validate(&plan);
            assert_eq!(report, validator.validate(&plan), "deterministic");
            invalid += usize::from(report.status() == ValidationStatus::Invalid);
        }
    }
    assert!(invalid > 0);
}

/// A tank with continuous filling and draining, an at end duration bound, `forall`
/// in conditions and effects, and costs.
const TANK: &str = "
(define (domain tank) (:requirements :durative-actions :continuous-effects :numeric-fluents)
  (:predicates (done) (item ?x))
  (:functions (level) (total-cost))
  (:durative-action fill :parameters ()
    :duration (and (>= ?duration 0.5) (at end (<= ?duration 10)))
    :condition (over all (or (<= (level) 2) (>= (level) 4)))
    :effect (and (increase (level) #t) (at end (done)) (at start (increase (total-cost) 1))))
  (:durative-action drain :parameters ()
    :duration (>= ?duration 1)
    :condition (over all (> (level) 0))
    :effect (and (decrease (level) (* 2 #t)) (decrease (level) #t) (at end (done))))
  (:durative-action tag :parameters ()
    :duration (= ?duration 1)
    :condition (forall (?x) (at start (item ?x)))
    :effect (and (forall (?x) (at end (not (item ?x)))) (at end (done))
                 (at start (increase (total-cost) 2)))))";

/// The tank at `level`, with items a and b unless `items` says otherwise.
fn tank(level: &str, items: &str) -> Task {
    Task::new(
        TANK,
        &format!(
            "(define (problem p) (:domain tank) (:objects a b)
               (:init (= (level) {level}) (= (total-cost) 0) {items})
               (:goal (done)) (:metric minimize (total-cost)))"
        ),
    )
}

#[test]
fn disjunctive_invariant_under_continuous_change() {
    let task = tank("1", "(item a) (item b)");
    assert_eq!(task.run("0: (fill) [1]").status(), ValidationStatus::Valid);

    // From 1 to 4 the level passes through (2, 4), where neither side holds.
    let kind = task.step_failure("0: (fill) [3]", 1);
    let ViolationKind::InvariantViolated { time, .. } = kind else {
        panic!("{kind:?}");
    };
    assert!(time > 1.0 && time < 3.0, "{time}");

    // Above 4 the invariant holds throughout, so the at end bound decides.
    let kind = tank("5", "").step_failure("0: (fill) [11]", 1);
    assert!(
        matches!(&kind, ViolationKind::DurationConstraint(Unmet::Comparison { condition, .. }) if condition == "(<= ?duration 10)"),
        "{kind:?}"
    );
}

#[test]
fn strict_invariant_reaching_its_bound_at_the_end() {
    // Two flows drain 3 per time unit: from 3 to exactly 0 at the end.
    assert_eq!(
        tank("3", "").run("0: (drain) [1]").status(),
        ValidationStatus::Valid
    );
    let kind = tank("2", "").step_failure("0: (drain) [1]", 1);
    assert!(
        matches!(kind, ViolationKind::InvariantViolated { .. }),
        "{kind:?}"
    );
}

#[test]
fn simultaneous_additive_updates_combine() {
    let report = tank("1", "(item a) (item b)").run("0: (fill) [1]\n0: (tag) [1]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.metric().unwrap().value, 3.0);
}

#[test]
fn forall_in_durative_conditions_and_effects() {
    let kind = tank("1", "(item a)").step_failure("0: (tag) [1]", 1);
    assert!(
        matches!(
            kind,
            ViolationKind::StartConditionUnsatisfied(Unmet::Atom { .. })
        ),
        "{kind:?}"
    );

    let report = tank("1", "(item a) (item b)").run("0: (tag) [1]");
    let end = report.trace().last().unwrap();
    assert_eq!(end.deleted.len(), 2);
}

#[test]
fn degenerate_times_and_durations() {
    let task = tank("3", "");
    let validator = task.validator();
    let run =
        |step: pddl_validate::PlanStep| validator.validate(&pddl_validate::Plan::new(vec![step]));
    let drain = || pddl_validate::PlanStep::new("drain", Vec::<String>::new());

    let kind = |report: pddl_validate::Report| report.violation().unwrap().kind.clone();
    assert!(matches!(
        kind(run(drain().at(f64::NAN).lasting(1.0))),
        ViolationKind::InvalidTime(_)
    ));
    assert!(matches!(
        kind(run(drain().at(f64::INFINITY).lasting(1.0))),
        ViolationKind::InvalidTime(_)
    ));
    assert!(matches!(
        kind(run(drain().at(0.0).lasting(f64::NAN))),
        ViolationKind::NonPositiveDuration(_)
    ));
    assert!(matches!(
        kind(run(drain().at(0.0).lasting(1e-10))),
        ViolationKind::NonPositiveDuration(_)
    ));
    assert!(matches!(
        task.step_failure("1e308: (drain) [1e308]", 1),
        ViolationKind::InvalidTime(_)
    ));
}

/// Two instantaneous actions, where `light` reads what `flip` adds.
const SWITCH: &str = "
(define (domain switch) (:predicates (on) (lit))
  (:action flip :parameters () :precondition (not (on)) :effect (on))
  (:action light :parameters () :precondition (on) :effect (lit)))";

#[test]
fn timed_plans_for_classical_domains() {
    let task = Task::new(
        SWITCH,
        "(define (problem p) (:domain switch) (:init) (:goal (lit)))",
    );
    let report = task.run("0: (flip)\n1: (light)");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    for plan in ["0: (flip)\n0: (light)", "0: (flip)\n0.005: (light)"] {
        let kind = task.step_failure(plan, 2);
        assert!(
            matches!(kind, ViolationKind::Interference { .. }),
            "{plan}: {kind:?}"
        );
    }

    assert_eq!(
        task.step_failure("(flip) [1]", 1),
        ViolationKind::UnexpectedDuration
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
fn random_temporal_plans_never_panic_or_hang() {
    const STEPS: &[&str] = &["(fill)", "(drain)", "(tag)", "(fill x)", "(nope)", "(tag"];
    const NUMBERS: &[&str] = &[
        "0", "0.5", "1", "1.005", "2", "3", "1e308", "1e-12", "10", "0.01",
    ];
    let task = tank("3", "(item a) (item b)");
    let validator = task.validator();
    let mut random = Lcg(11);
    for _ in 0..1500 {
        let mut text = String::new();
        for _ in 0..random.below(5) {
            let time = NUMBERS[random.below(NUMBERS.len())];
            let step = STEPS[random.below(STEPS.len())];
            let duration = NUMBERS[random.below(NUMBERS.len())];
            match random.below(3) {
                0 => writeln!(text, "{time}: {step} [{duration}]").unwrap(),
                1 => writeln!(text, "{time}: {step}").unwrap(),
                _ => writeln!(text, "{step} [{duration}]").unwrap(),
            }
        }

        let _ = validator.validate_text(&text);
    }
}

#[test]
fn a_start_time_too_large_to_hold_the_duration_fails() {
    let domain = "(define (domain d) (:requirements :durative-actions :negative-preconditions)
        (:predicates (q))
        (:durative-action bad :parameters () :duration (= ?duration 2)
          :condition (and (at end (not (q))) (over all (not (q))))
          :effect (at start (q))))";
    let problem = "(define (problem p) (:domain d) (:init) (:goal (and)))";
    let task = Task::new(domain, problem);
    assert_eq!(task.run("0: (bad) [2]").status(), ValidationStatus::Invalid);
    assert_eq!(
        task.step_failure("100000000000000000: (bad) [2]", 1),
        ViolationKind::NonPositiveDuration(0.0)
    );
}

/// A domain whose `go` lasts a third of `(dist)`, and whose `loop` lasts 2.
const ROUNDED: &str = "(define (domain d) (:requirements :durative-actions :numeric-fluents)
    (:predicates (done)) (:functions (dist))
    (:durative-action go :parameters () :duration (= ?duration (/ (dist) 3))
      :effect (at end (done)))
    (:durative-action loop :parameters (?x) :duration (= ?duration 2)
      :effect (at end (done))))";

/// A problem for [`ROUNDED`] with `(dist)` 10 and objects `a` and `b`.
const ROUNDED_PROBLEM: &str =
    "(define (problem p) (:domain d) (:objects a b) (:init (= (dist) 10)) (:goal (done)))";

#[test]
fn durations_rounded_by_the_planner_satisfy_their_constraint() {
    // 10 / 3 printed with three decimals is 0.00033 below the exact duration.
    let task = Task::new(ROUNDED, ROUNDED_PROBLEM);
    assert_eq!(
        task.run("0: (go) [3.333]").status(),
        ValidationStatus::Valid
    );
    assert_eq!(
        task.run("0: (go) [3.3333333]").status(),
        ValidationStatus::Valid
    );
    // 3.334 is not 10 / 3 rounded to three decimals.
    assert!(matches!(
        task.step_failure("0: (go) [3.334]", 1),
        ViolationKind::DurationConstraint(_)
    ));

    let exact = task.validator().with_duration_tolerance(0.0);
    assert_eq!(
        exact.validate_text("0: (go) [3.333]").status(),
        ValidationStatus::Invalid
    );
}

#[test]
fn a_ground_durative_action_must_not_overlap_itself() {
    let task = Task::new(ROUNDED, ROUNDED_PROBLEM);
    assert_eq!(
        task.step_failure("0: (loop a) [2]\n1: (loop a) [2]", 2),
        ViolationKind::SelfOverlap { other: 1 }
    );
    assert_eq!(
        task.step_failure("0: (loop a) [2]\n0: (LOOP A) [2]", 2),
        ViolationKind::SelfOverlap { other: 1 }
    );
    // Plan order does not matter: the later step in the file starts first.
    assert_eq!(
        task.step_failure("1: (loop a) [2]\n0: (loop a) [2]", 2),
        ViolationKind::SelfOverlap { other: 1 }
    );

    // Different arguments, or one instance ending where the next starts, are fine.
    assert_eq!(
        task.run("0: (loop a) [2]\n1: (loop b) [2]").status(),
        ValidationStatus::Valid
    );
    assert_eq!(
        task.run("0: (loop a) [2]\n2: (loop a) [2]").status(),
        ValidationStatus::Valid
    );
}

/// A durative action whose end adds what an instant action reads.
const HOLD: &str = "
(define (domain hold)
  (:requirements :durative-actions)
  (:predicates (held) (used))
  (:durative-action hold :parameters () :duration (>= ?duration 0)
    :condition () :effect (at end (held)))
  (:action useg :parameters () :precondition (held) :effect (used)))";

/// A problem for [`HOLD`].
const HOLD_PROBLEM: &str = "(define (problem p) (:domain hold) (:init) (:goal (used)))";

#[test]
fn epsilon_separation_holds_at_large_times() {
    let task = Task::new(HOLD, HOLD_PROBLEM);
    // Exactly epsilon apart as written, whatever the rounding of the times.
    for start in [0.0, 100_000.0, 1_000_000_000.0] {
        let plan = format!("{start}: (hold) [2]\n{}.01: (useg)", start + 2.0);
        let report = task.run(&plan);
        assert_eq!(
            report.status(),
            ValidationStatus::Valid,
            "{plan}: {report:?}"
        );
    }

    // Closer than epsilon still interferes at large times.
    let kind = task.step_failure("1000000000: (hold) [2]\n1000000002.005: (useg)", 2);
    assert!(
        matches!(kind, ViolationKind::Interference { .. }),
        "{kind:?}"
    );
}

/// A lab whose durative actions are `actions`, over the items a and b.
fn lab(actions: &str, init: &str, goal: &str) -> Task {
    Task::new(
        &format!(
            "(define (domain lab)
               (:requirements :typing :durative-actions :numeric-fluents :conditional-effects
                              :universal-preconditions :preferences)
               (:types item)
               (:predicates (p) (q) (r) (s) (done ?i - item))
               (:functions (x) (y))
               {actions})"
        ),
        &format!(
            "(define (problem p) (:domain lab) (:objects a b - item)
               (:init (= (x) 0) {init}) (:goal {goal}))"
        ),
    )
}

/// Asserts that every plan for `task` is unsupported because of `construct`.
fn assert_unsupported(task: &Task, plan: &str, construct: &Construct) {
    let report = task.run(plan);
    assert_eq!(report.status(), ValidationStatus::Unsupported, "{report:?}");
    assert_eq!(report.unsupported().unwrap().construct, *construct);
}

#[test]
fn an_at_end_duration_bound_reads_the_state_at_the_end() {
    // At the start x is 0, so the bound would fail there; at the end x is 2.
    let work = "(:durative-action work :parameters ()
                  :duration (and (>= ?duration 1) (at end (<= ?duration (x))))
                  :effect (and (at start (assign (x) 2)) (at end (p))))";
    let task = lab(work, "", "(p)");
    let report = task.run("0: (work) [2]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");

    let kind = task.step_failure("0: (work) [3]", 1);
    assert_eq!(
        kind,
        ViolationKind::DurationConstraint(Unmet::Comparison {
            condition: "(<= ?duration (x))".into(),
            left: 3.0,
            right: 2.0,
            expected: true
        })
    );
}

#[test]
fn a_durative_action_needs_neither_a_duration_constraint_nor_an_effect() {
    let idle = "(:durative-action idle :parameters () :duration ()
                  :condition (at start (q)) :effect ())";
    let task = lab(idle, "(q)", "(q)");
    let report = task.run("0: (idle) [7]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.trace().len(), 2);
    assert!(report.trace().iter().all(|h| h.added.is_empty()));

    let task = lab(idle, "", "(q)");
    let kind = task.step_failure("0: (idle) [7]", 1);
    assert!(
        matches!(kind, ViolationKind::StartConditionUnsatisfied(_)),
        "{kind:?}"
    );
}

#[test]
fn effects_compute_with_the_duration() {
    // With d = 2: at the start x = 0 - (-d) = 2, at the end x = 2 + 2d = 6, y = d + 1 + 2.
    let measure = "(:durative-action measure :parameters () :duration (>= ?duration 1)
                     :effect (and (at start (decrease (x) (- ?duration)))
                                  (at end (increase (x) (* 2 ?duration)))
                                  (at end (assign (y) (+ ?duration 1 2)))))";
    let task = lab(measure, "", "(and (= (x) 6) (= (y) 5))");
    let report = task.run("0: (measure) [2]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(task.violation("0: (measure) [3]").step, None);
}

#[test]
fn a_duration_expression_divides_by_zero() {
    let split = "(:durative-action split :parameters () :duration (>= ?duration 1)
                   :effect (at end (assign (x) (/ 1 (- ?duration 2)))))";
    let task = lab(split, "", "(>= (x) 0)");
    assert_eq!(
        task.step_failure("0: (split) [2]", 1),
        ViolationKind::DivisionByZero("?duration expression".into())
    );
    assert_eq!(task.run("0: (split) [3]").status(), ValidationStatus::Valid);
}

#[test]
fn conditional_effects_of_durative_actions() {
    // An `at start` effect under an `at start` condition, and an `at end` effect under
    // a conjunction with a `forall` over the end state.
    let mark = "(:durative-action mark :parameters () :duration (= ?duration 1)
                  :effect (and (when (at start (q)) (at start (and (p) (r))))
                               (when (and (at start (q))
                                          (forall (?i - item) (at end (not (done ?i)))))
                                     (at end (s)))))";
    let goal = "(and (p) (r) (s))";
    let report = lab(mark, "(q)", goal).run("0: (mark) [1]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
    assert_eq!(report.trace()[0].added.len(), 2, "{report:?}");

    let violation = lab(mark, "", goal).violation("0: (mark) [1]");
    assert!(
        matches!(violation.kind, ViolationKind::GoalUnsatisfied(_)),
        "{violation:?}"
    );

    let violation = lab(mark, "(q) (done a)", "(s)").violation("0: (mark) [1]");
    assert!(
        matches!(violation.kind, ViolationKind::GoalUnsatisfied(_)),
        "{violation:?}"
    );
}

#[test]
fn unsupported_durative_constructs() {
    // A condition on continuous change has no single moment to be checked at.
    let flow = "(:durative-action flow :parameters () :duration (= ?duration 1)
                  :effect (when (at start (q)) (increase (x) (* #t 1))))";
    assert_unsupported(
        &lab(flow, "", "(q)"),
        "0: (flow) [1]",
        &Construct::TimedEffectConditions,
    );

    let wish = "(:durative-action wish :parameters () :duration (= ?duration 1)
                  :condition (preference early (at start (q))) :effect (at end (p)))";
    assert_unsupported(
        &lab(wish, "", "(p)"),
        "0: (wish) [1]",
        &Construct::Preferences,
    );
}

#[test]
fn an_end_effect_on_an_undefined_fluent_fails_at_the_end() {
    let count = "(:durative-action count :parameters () :duration (= ?duration 1)
                   :effect (and (at start (p)) (at end (increase (y) 1))))";
    let task = lab(count, "", "(p)");
    let violation = task.violation("0: (count) [1]");
    assert!(
        matches!(&violation.kind, ViolationKind::Undefined(fluent) if fluent.to_string() == "(y)"),
        "{violation:?}"
    );
    assert_eq!(violation.step.unwrap().number, 1);

    let report = lab(count, "(= (y) 0)", "(= (y) 1)").run("0: (count) [1]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
}
