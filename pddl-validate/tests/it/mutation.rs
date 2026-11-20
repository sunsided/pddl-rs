//! Mutations of valid plans: dropping, swapping and repeating steps.
//!
//! Dropping a step of an optimal unit-cost plan always makes it invalid, since a valid
//! shorter plan would contradict optimality. Swaps and repeats may stay valid, so for
//! them the tests check only that validation never panics and is deterministic.

use pddl_validate::{Plan, PlanStep, ValidationStatus};

use crate::classical::{LAMP, MOVE_ONE, MOVE_ONE_PLAN, TRANSPORT, WIRING};
use crate::Task;

/// Returns every plan with one step dropped.
fn drops(plan: &Plan) -> Vec<Plan> {
    (0..plan.len())
        .map(|skip| {
            plan.steps()
                .iter()
                .enumerate()
                .filter(|(i, _)| *i != skip)
                .map(|(_, s)| s.clone())
                .collect()
        })
        .collect()
}

/// Returns every plan with two adjacent steps swapped, and every plan with one step
/// repeated.
fn swaps_and_repeats(plan: &Plan) -> Vec<Plan> {
    let steps = plan.steps();
    let mut out = Vec::new();
    for i in 0..steps.len().saturating_sub(1) {
        let mut swapped = steps.to_vec();
        swapped.swap(i, i + 1);
        out.push(Plan::new(swapped));
    }

    for i in 0..steps.len() {
        let mut repeated: Vec<PlanStep> = steps.to_vec();
        repeated.insert(i, steps[i].clone());
        out.push(Plan::new(repeated));
    }

    out
}

/// Checks the mutations of an optimal valid plan.
fn check_mutations(task: &Task, plan: &str) {
    let validator = task.validator();
    let plan: Plan = plan.parse().unwrap();
    assert_eq!(validator.validate(&plan).status(), ValidationStatus::Valid);

    for dropped in drops(&plan) {
        let report = validator.validate(&dropped);
        assert_eq!(report.status(), ValidationStatus::Invalid, "{dropped:?}");
    }

    let mutants = swaps_and_repeats(&plan);
    let invalid = mutants
        .iter()
        .filter(|mutant| {
            let report = validator.validate(mutant);
            assert_eq!(report, validator.validate(mutant), "deterministic");
            report.status() == ValidationStatus::Invalid
        })
        .count();
    assert!(invalid > 0, "some swap or repeat breaks the plan");
}

#[test]
fn transport() {
    check_mutations(&Task::new(TRANSPORT, MOVE_ONE), MOVE_ONE_PLAN);
}

#[test]
fn wiring() {
    check_mutations(
        &Task::new(WIRING, LAMP),
        "(lay plant junction)\n(lay junction lamp)",
    );
}
