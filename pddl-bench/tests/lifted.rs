//! Generated code (level B1) agrees with the runtime grounder: the same
//! applicable actions in every state, and the same optimal plan costs.

use pddl_bench::check::lockstep;
use pddl_bench::domains::{blocks, gripper, logistics};
use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::{astar, breadth_first, replay};
use pddl_runtime::Task;

/// The grounder's operator name in PDDL syntax, as the generated code writes it.
fn ground_name(task: &GroundTask, op: u32) -> String {
    format!("({})", task.name(op))
}

#[test]
fn gripper_matches_the_grounder() {
    for (name, optimum) in [("gripper-2", 3), ("gripper-4", 9)] {
        let (domain, problem) = fixture(name);
        let grounded = GroundTask::new(&domain, &problem).unwrap();
        let task = gripper::Task::new(&problem).unwrap();
        let visited = lockstep(
            &task,
            |a| task.describe(a),
            &grounded,
            |a| ground_name(&grounded, a),
            usize::MAX,
        );
        assert!(visited > 10, "{name}: {visited}");

        let plan = breadth_first(&task).plan.unwrap();
        assert_eq!(plan.cost, optimum, "{name}");
        assert_eq!(replay(&task, &plan.actions), Some(optimum), "{name}");
        assert_eq!(
            breadth_first(&grounded).plan.unwrap().cost,
            optimum,
            "{name}"
        );
    }
}

#[test]
fn larger_domains_match_the_grounder() {
    let (domain, problem) = fixture("bw-large-a");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    let task = blocks::Task::new(&problem).unwrap();
    assert_eq!(
        lockstep(
            &task,
            |a| task.describe(a),
            &grounded,
            |a| ground_name(&grounded, a),
            3000
        ),
        3000
    );

    let (domain, problem) = fixture("logistics-e");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    let task = logistics::Task::new(&problem).unwrap();
    assert_eq!(
        lockstep(
            &task,
            |a| task.describe(a),
            &grounded,
            |a| ground_name(&grounded, a),
            3000
        ),
        3000
    );
}

#[test]
fn goal_count_guides_a_star_on_generated_code() {
    let (_, problem) = fixture("bw-large-a");
    let task = blocks::Task::new(&problem).unwrap();
    assert!(task.unsatisfied_goals(&task.initial_state()) > 0);
    let plan = astar(&task, |s| task.unsatisfied_goals(s)).plan.unwrap();
    assert!(replay(&task, &plan.actions).is_some());
}

#[test]
fn problems_for_other_domains_are_rejected() {
    let (_, problem) = fixture("gripper-2");
    assert!(matches!(
        blocks::Task::new(&problem),
        Err(pddl::codegen::LoadError::WrongDomain { .. })
    ));
}
