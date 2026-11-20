//! Generated code with numeric fluents and action costs (#129).

use pddl::codegen::LoadError;
use pddl::{Parser, Problem};
use pddl_bench::check::lockstep;
use pddl_bench::domains::{courier, fuel, gripper_costs};
use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::{astar, breadth_first, ida_star, replay};
use pddl_runtime::Task;

#[test]
fn action_costs_match_the_grounder() {
    let (domain, problem) = fixture("gripper-costs");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    let task = gripper_costs::Task::new(&problem).unwrap();
    lockstep(
        &task,
        |a| task.describe(a),
        &grounded,
        |a| format!("({})", grounded.name(a)),
        usize::MAX,
    );
    for plan in [astar(&task, |_| 0).plan, ida_star(&task, |_| 0).plan] {
        let plan = plan.unwrap();
        assert_eq!(plan.cost, 7);
        assert_eq!(replay(&task, &plan.actions), Some(7));
    }

    let (domain, problem) = fixture("courier");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    let task = courier::Task::new(&problem).unwrap();
    lockstep(
        &task,
        |a| task.describe(a),
        &grounded,
        |a| format!("({})", grounded.name(a)),
        usize::MAX,
    );
    let plan = astar(&task, |_| 0).plan.unwrap();
    assert_eq!(plan.cost, 16);
    assert_eq!(replay(&task, &plan.actions), Some(16));
    // The grounder gives each operator the same cost.
    let mut out = Vec::new();
    task.successors(&task.initial_state(), &mut out);
    let mut ground_out = Vec::new();
    grounded.successors(&grounded.initial_state(), &mut ground_out);
    for (action, _) in &out {
        let name = task.describe(*action);
        let (op, _) = ground_out
            .iter()
            .find(|(op, _)| format!("({})", grounded.name(*op)) == name)
            .unwrap();
        assert_eq!(task.cost(*action), grounded.cost(*op), "{name}");
    }
}

/// a -3- b -4- c, a -9- c; a station at b only; tank of 5, starting with `start`.
fn fuel_task(start: &str, extra: &str) -> Result<fuel::Task, LoadError> {
    let problem = Problem::from_str(&format!(
        "(define (problem p) (:domain fuel) (:objects a b c - place)
           (:init (at a) (station b) (road a b) (road b c) (road a c)
                  (= (need a b) 3) (= (need b c) 4) (= (need a c) 9)
                  {start} (= (capacity) 5) (= (total-cost) 0) {extra})
           (:goal (at c)) (:metric minimize (total-cost)))"
    ))
    .unwrap();
    fuel::Task::new(&problem)
}

#[test]
fn numeric_preconditions_and_effects_follow_pddl() {
    // a -> b (3), refuel (1), b -> c (4): cost 8. a -> c needs 9 > 5 fuel.
    let task = fuel_task("(= (fuel) 5)", "").unwrap();
    let plan = astar(&task, |_| 0).plan.unwrap();
    assert_eq!(plan.cost, 8);
    let names: Vec<String> = plan.actions.iter().map(|&a| task.describe(a)).collect();
    assert_eq!(names, ["(drive a b)", "(refuel b)", "(drive b c)"]);
    assert_eq!(replay(&task, &plan.actions), Some(8));
    // Without the metric every action costs 1.
    assert_eq!(breadth_first(&task).plan.unwrap().actions.len(), 3);
}

#[test]
fn undefined_values_make_actions_inapplicable() {
    // Fuel undefined: no comparison holds, not even negated, so no action applies.
    let task = fuel_task("", "").unwrap();
    let mut out = Vec::new();
    task.successors(&task.initial_state(), &mut out);
    assert!(
        out.is_empty(),
        "{:?}",
        out.iter()
            .map(|(a, _)| task.describe(*a))
            .collect::<Vec<_>>()
    );
    assert_eq!(astar(&task, |_| 0).plan, None);
}

#[test]
fn cost_functions_must_hold_non_negative_integers() {
    assert_eq!(
        fuel_task("(= (fuel) 5)", "(= (need c a) 1.5)").unwrap_err(),
        LoadError::InvalidCost {
            function: "need".into(),
            value: 1.5
        }
    );
}
