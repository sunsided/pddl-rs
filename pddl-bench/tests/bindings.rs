//! Bound types and semantic attachments (#131).

use pddl::{Parser, Problem};
use pddl_bench::domains::{rovers, Cell};
use pddl_runtime::search::{astar, replay};
use pddl_runtime::Task;

/// Places `p<x><y>` on a 3x2 grid; the rover starts at p00, the goal is p21.
fn task() -> rovers::Task {
    let problem = Problem::from_str(
        "(define (problem p) (:domain rovers) (:objects r - rover p00 p10 p20 p01 p11 p21 - place)
           (:init (at r p00) (= (total-cost) 0)) (:goal (at r p21)) (:metric minimize (total-cost)))",
    )
    .unwrap();
    rovers::Task::new(&problem, |name| {
        let digit = |i: usize| i32::from(name.as_bytes()[i] - b'0');
        Cell {
            x: digit(1),
            y: digit(2),
        }
    })
    .unwrap()
}

#[test]
fn attachments_decide_preconditions_and_costs() {
    let task = task();
    assert_eq!(task.bound().place.len(), 6);
    let mut out = Vec::new();
    task.successors(&task.initial_state(), &mut out);
    let mut names: Vec<String> = out.iter().map(|(a, _)| task.describe(*a)).collect();
    names.sort();
    assert_eq!(names, ["(go r p00 p01)", "(go r p00 p10)"]);
    // Two steps along the row (1 each) and one across (2): 4.
    let plan = astar(&task, |_| 0).plan.unwrap();
    assert_eq!(plan.cost, 4);
    assert_eq!(replay(&task, &plan.actions), Some(4));
}
