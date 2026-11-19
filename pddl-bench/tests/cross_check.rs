//! Every implementation (runtime grounder, B1, B2) and every optimal search
//! (breadth-first, A* and IDA* with `h = 0`) agree on the optimal plan cost,
//! and every plan replays (#125).

use pddl::{Parser, Problem};
use pddl_bench::domains::{
    gripper, gripper_2_grounded, gripper_4_grounded, logistics, logistics_small_grounded,
};
use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::{astar, breadth_first, ida_star, replay};
use pddl_runtime::Task;

/// Solves `task` with each optimal search and returns the costs, after
/// checking that each plan replays at its cost.
fn optimal_costs<T: Task>(task: &T) -> [u64; 3] {
    [
        breadth_first(task),
        astar(task, |_| 0),
        ida_star(task, |_| 0),
    ]
    .map(|outcome| {
        let plan = outcome.plan.expect("solvable");
        assert_eq!(replay(task, &plan.actions), Some(plan.cost));
        plan.cost
    })
}

#[test]
fn implementations_and_searches_agree() {
    for (name, optimum) in [("gripper-2", 3), ("gripper-4", 9)] {
        let (domain, problem) = fixture(name);
        let expected = [optimum; 3];
        assert_eq!(
            optimal_costs(&GroundTask::new(&domain, &problem).unwrap()),
            expected,
            "{name}: grounder"
        );
        assert_eq!(
            optimal_costs(&gripper::Task::new(&problem).unwrap()),
            expected,
            "{name}: B1"
        );
        let b2 = if name == "gripper-2" {
            optimal_costs(&gripper_2_grounded::Task)
        } else {
            optimal_costs(&gripper_4_grounded::Task)
        };
        assert_eq!(b2, expected, "{name}: B2");
    }

    // A small logistics problem: trucks, a plane and two cities.
    let (domain, _) = fixture("logistics-e");
    let problem = Problem::from_str(include_str!("../fixtures/logistics-small.pddl")).unwrap();
    let expected = [9; 3];
    assert_eq!(
        optimal_costs(&GroundTask::new(&domain, &problem).unwrap()),
        expected,
        "logistics: grounder"
    );
    assert_eq!(
        optimal_costs(&logistics::Task::new(&problem).unwrap()),
        expected,
        "logistics: B1"
    );
    assert_eq!(
        optimal_costs(&logistics_small_grounded::Task),
        expected,
        "logistics: B2"
    );
}
