//! The runtime grounder finds the optimal plans that Fast Downward's
//! translator and an optimal search agree on (see `tests/data/sas-plus`).

use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::{astar, breadth_first, ida_star, replay};
use pddl_runtime::Task;

/// The fixtures with their optimal plan costs.
const OPTIMA: [(&str, u64); 6] = [
    ("sussman-anomaly", 3),
    ("gripper-2", 3),
    ("gripper-4", 9),
    ("gripper-costs", 7),
    ("courier", 16),
    // Not in the README; `sas-plus`'s exhaustive `perfect_heuristic` gives 6.
    ("get-paid4", 6),
];

#[test]
fn optimal_searches_find_the_known_optima() {
    for (name, optimum) in OPTIMA {
        let (domain, problem) = fixture(name);
        let task = GroundTask::new(&domain, &problem).unwrap();
        for (search, outcome) in [
            ("A*", astar(&task, |_| 0)),
            ("IDA*", ida_star(&task, |_| 0)),
        ] {
            let plan = outcome
                .plan
                .unwrap_or_else(|| panic!("{name}: {search} found no plan"));
            assert_eq!(plan.cost, optimum, "{name}: {search}");
            assert_eq!(
                replay(&task, &plan.actions),
                Some(optimum),
                "{name}: {search}"
            );
        }
    }
}

#[test]
fn breadth_first_plans_replay() {
    for (name, optimum) in OPTIMA {
        let (domain, problem) = fixture(name);
        let task = GroundTask::new(&domain, &problem).unwrap();
        let plan = breadth_first(&task).plan.unwrap();
        let cost = replay(&task, &plan.actions).unwrap();
        assert!(cost >= optimum, "{name}");
        let names: Vec<_> = plan.actions.iter().map(|&a| task.name(a)).collect();
        assert!(!names.is_empty(), "{name}");
    }
}

#[test]
fn goal_count_guides_a_star_to_a_valid_plan() {
    let (domain, problem) = fixture("gripper-4");
    let task = GroundTask::new(&domain, &problem).unwrap();
    assert!(task.unsatisfied_goals(&task.initial_state()) > 0);
    let plan = astar(&task, |s| task.unsatisfied_goals(s)).plan.unwrap();
    assert!(replay(&task, &plan.actions).is_some());
}
