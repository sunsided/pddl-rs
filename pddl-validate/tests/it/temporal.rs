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
