//! Rounding at the edges of continuous change: roots that round onto an interval end,
//! and continuous change that leaves the finite range.

use pddl_validate::{ValidationStatus, ViolationKind};

use super::Task;

/// A domain where `drain` lowers `(level)` at `(rate)` per time unit while
/// `(> (level) 0)` must hold over all, and `pump` raises `(f)` at `10^308` per time unit.
const DOMAIN: &str = "(define (domain rounding)
  (:requirements :durative-actions :continuous-effects :numeric-fluents)
  (:functions (level) (rate) (f))
  (:durative-action drain
    :parameters ()
    :duration (<= ?duration 10)
    :condition (over all (> (level) 0))
    :effect (decrease (level) (* #t (rate))))
  (:durative-action pump
    :parameters ()
    :duration (<= ?duration 10)
    :condition (at start (>= (f) 0))
    :effect (increase (f) (* #t BIG))))";

/// Builds the task with `(level)` and `(rate)` set as given, and `goal` as the goal.
fn task(level: &str, rate: &str, goal: &str) -> Task {
    let big = format!("1{}", "0".repeat(308));
    Task::new(
        &DOMAIN.replace("BIG", &big),
        &format!(
            "(define (problem p) (:domain rounding)
               (:init (= (level) {level}) (= (rate) {rate}) (= (f) 0))
               (:goal {goal}))"
        ),
    )
}

#[test]
fn root_rounding_onto_the_interval_end_is_not_sampled() {
    // 0.3 - 3 * 0.1 is -5.6e-17, so the root computes as 2.9999999999999996, where the
    // level is 5.6e-17: equal to 0 within the tolerance, but strictly inside the interval.
    let report = task("0.3", "0.1", "(>= (level) 0)").run("0: (drain) [3]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
}

#[test]
fn crossing_well_inside_the_interval_is_caught() {
    let kind = task("0.3", "0.1", "(>= (rate) 0)").step_failure("0: (drain) [5]", 1);
    let ViolationKind::InvariantViolated { time, .. } = kind else {
        panic!("{kind:?}");
    };
    assert!(time > 2.9 && time < 5.0, "{time}");
}

#[test]
fn continuous_overflow_is_invalid() {
    // The advance to the end of `pump` overflows, not a later read of `(f)`.
    let kind = task("1", "0", "(= (f) 5)").step_failure("0: (pump) [10]", 1);
    assert_eq!(kind, ViolationKind::NonFinite("(f)".into()));

    // A true branch of an `or` cannot hide the overflow.
    let kind = task("1", "0", "(or (= (f) 5) (> (level) 0))").step_failure("0: (pump) [10]", 1);
    assert_eq!(kind, ViolationKind::NonFinite("(f)".into()));
}

#[test]
fn a_process_root_rounding_onto_the_segment_end_is_not_a_switch() {
    // `drain` stops where the level reaches 0, exactly at 10^7, the end of `watch`. The
    // root computes about 1.9e-9 earlier, beyond the same-time window but within the
    // rounding slack; a switch there would check `watch`'s invariant at a level of 0.
    let task = Task::new(
        "(define (domain sink) (:requirements :time :durative-actions)
           (:predicates (seen))
           (:functions (level))
           (:durative-action watch :parameters () :duration (= ?duration 10000000)
             :condition (over all (> (level) 0)) :effect (at end (seen)))
           (:process drain :parameters () :precondition (> (level) 0)
             :effect (decrease (level) (* #t 0.07))))",
        "(define (problem p) (:domain sink) (:init (= (level) 700000)) (:goal (seen)))",
    );
    let report = task.run("0: (watch) [10000000]");
    assert_eq!(report.status(), ValidationStatus::Valid, "{report:?}");
}
