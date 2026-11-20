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

#[test]
fn cost_functions_may_hold_any_number_without_the_metric() {
    let problem = Problem::from_str(
        "(define (problem p) (:domain fuel) (:objects a b - place)
           (:init (at a) (road a b) (= (need a b) 2.5) (= (fuel) 5) (= (capacity) 5))
           (:goal (at b)))",
    )
    .unwrap();
    let task = fuel::Task::new(&problem).unwrap();
    assert_eq!(breadth_first(&task).plan.unwrap().actions.len(), 1);
}

/// Each action reads `f`, which may be undefined or huge, in a condition whose
/// value must not depend on the order of its parts.
#[pddl::task(domain_source = r"
(define (domain order)
  (:requirements :adl :typing :numeric-fluents)
  (:types thing)
  (:predicates (p) (q) (done))
  (:functions (f) (g ?t - thing))
  (:action or-first :parameters () :precondition (or (> (f) 0) (p)) :effect (done))
  (:action or-last :parameters () :precondition (or (p) (> (f) 0)) :effect (done))
  (:action nand-first :parameters () :precondition (not (and (> (f) 0) (q))) :effect (done))
  (:action nand-last :parameters () :precondition (not (and (q) (> (f) 0))) :effect (done))
  (:action exists-g :parameters () :precondition (exists (?t - thing) (> (g ?t) 0)) :effect (done))
  (:action when-or :parameters () :precondition (p)
    :effect (when (or (p) (> (f) 0)) (done)))
  (:action square :parameters () :precondition (> (* (f) (f)) 0) :effect (done))
  (:action overflow :parameters () :precondition (p) :effect (scale-up (f) (f))))
")]
mod order {}

/// An `order` problem: `(p)` holds, `(q)` doesn't, `g` is 1 at `t1` and
/// undefined at `t2`; `f` is set by `init`, and the goal is `goal`.
fn order_task(init: &str, goal: &str) -> order::Task {
    let problem = Problem::from_str(&format!(
        "(define (problem x) (:domain order) (:objects t1 t2 - thing)
           (:init (p) (= (g t1) 1) {init}) (:goal {goal}))"
    ))
    .unwrap();
    order::Task::new(&problem).unwrap()
}

/// The applicable actions of the initial state, and whether `when-or` adds `(done)`.
fn applicable(task: &order::Task) -> (Vec<String>, bool) {
    let mut out = Vec::new();
    task.successors(&task.initial_state(), &mut out);
    let done = out
        .iter()
        .any(|(a, next)| task.describe(*a) == "(when-or)" && next.done);
    let names = out.iter().map(|(a, _)| task.describe(*a)).collect();

    (names, done)
}

#[test]
fn undefined_values_fail_a_condition_in_any_order() {
    for goal in [
        "(or (> (f) 0) (p))",
        "(or (p) (> (f) 0))",
        "(not (and (q) (> (f) 0)))",
        "(not (and (> (f) 0) (q)))",
    ] {
        let task = order_task("", goal);
        assert!(!task.is_goal(&task.initial_state()), "{goal}");
    }
    let task = order_task("", "(done)");
    assert_eq!(applicable(&task), (vec!["(when-or)".to_string()], false));
}

#[test]
fn non_finite_values_are_undefined() {
    // 1e200: the parser takes no exponents.
    let task = order_task(
        &format!("(= (f) 1{})", "0".repeat(200)),
        "(> (* (f) (f)) 0)",
    );
    let (mut names, done) = applicable(&task);
    names.sort();
    assert_eq!(
        names,
        [
            "(nand-first)",
            "(nand-last)",
            "(or-first)",
            "(or-last)",
            "(when-or)"
        ]
    );
    assert!(done);
    assert!(!task.is_goal(&task.initial_state()));
}
