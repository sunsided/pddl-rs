//! Tasks grounded when the crate compiles (level B2, #127).

use pddl_bench::check::lockstep;
use pddl_bench::domains::{bw_large_a_grounded as bw, gripper_4_grounded as gripper};
use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::{astar, breadth_first, replay};
use pddl_runtime::Task;

#[test]
fn grounded_tasks_match_the_runtime_grounder() {
    let (domain, problem) = fixture("gripper-4");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    assert_eq!(gripper::OPERATORS.len(), grounded.len());
    let visited = lockstep(
        &gripper::Task,
        |a| gripper::NAMES[a.0 as usize].to_string(),
        &grounded,
        |a| format!("({})", grounded.name(a)),
        usize::MAX,
    );
    assert!(visited > 100, "{visited}");
    let plan = breadth_first(&gripper::Task).plan.unwrap();
    assert_eq!(plan.cost, 9);
    assert_eq!(replay(&gripper::Task, &plan.actions), Some(9));

    let (domain, problem) = fixture("bw-large-a");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    assert_eq!(bw::OPERATORS.len(), 648);
    assert_eq!(
        lockstep(
            &bw::Task,
            |a| bw::NAMES[a.0 as usize].to_string(),
            &grounded,
            |a| format!("({})", grounded.name(a)),
            3000
        ),
        3000
    );
    let plan = astar(&bw::Task, |s| bw::Task.unsatisfied_goals(s))
        .plan
        .unwrap();
    assert!(replay(&bw::Task, &plan.actions).is_some());
}
