//! Generated code with ADL (#128): quantifiers, disjunction, implication and
//! conditional effects.

use pddl::{Parser, Problem};
use pddl_bench::check::lockstep;
use pddl_bench::domains::{blocks_world, briefcase, lights};
use pddl_bench::{fixture, GroundTask};
use pddl_runtime::search::{astar, breadth_first, replay};
use pddl_runtime::Task;

#[test]
fn conditional_effects_match_the_grounder() {
    let (domain, problem) = fixture("sussman-anomaly");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    let task = blocks_world::Task::new(&problem).unwrap();
    let visited = lockstep(
        &task,
        |a| task.describe(a),
        &grounded,
        |a| format!("({})", grounded.name(a)),
        usize::MAX,
    );
    assert!(visited > 10, "{visited}");
    assert_eq!(breadth_first(&task).plan.unwrap().cost, 3);

    let (domain, problem) = fixture("get-paid4");
    let grounded = GroundTask::new(&domain, &problem).unwrap();
    let task = briefcase::Task::new(&problem).unwrap();
    let visited = lockstep(
        &task,
        |a| task.describe(a),
        &grounded,
        |a| format!("({})", grounded.name(a)),
        usize::MAX,
    );
    assert!(visited > 10, "{visited}");
    let plan = astar(&task, |_| 0).plan.unwrap();
    assert_eq!(plan.cost, 6);
    assert_eq!(replay(&task, &plan.actions), Some(6));
}

/// Rooms `r1` (switch `s1`), `r2` (switches `s2`, `s3`) and `r3` (none).
fn lights_problem(goal: &str) -> lights::Task {
    let problem = Problem::from_str(&format!(
        "(define (problem p) (:domain lights)
           (:objects s1 s2 s3 - switch r1 r2 r3 - room)
           (:init (in s1 r1) (in s2 r2) (in s3 r2))
           (:goal {goal}))"
    ))
    .unwrap();
    lights::Task::new(&problem).unwrap()
}

#[test]
fn quantifiers_and_disjunctions_follow_pddl() {
    // Light r1 and r2 (two flips, two lights), finish, then reset every
    // switch at once: 6 actions. r3 has no switch, so `finish` doesn't need it.
    let task = lights_problem("(and (done) (forall (?s - switch) (not (on ?s))))");
    let plan = breadth_first(&task).plan.unwrap();
    assert_eq!(plan.cost, 6);
    assert_eq!(replay(&task, &plan.actions), Some(6));
    let names: Vec<String> = plan.actions.iter().map(|&a| task.describe(a)).collect();
    assert_eq!(
        names.last().map(String::as_str),
        Some("(reset)"),
        "{names:?}"
    );

    // `light` needs a switch of its own room that is on.
    let initial = task.initial_state();
    let mut successors = Vec::new();
    task.successors(&initial, &mut successors);
    let mut names: Vec<String> = successors.iter().map(|(a, _)| task.describe(*a)).collect();
    names.sort();
    assert_eq!(names, ["(flip s1)", "(flip s2)", "(flip s3)", "(reset)"]);

    // An `or` goal; r3 can never be lit.
    let task = lights_problem("(or (lit r3) (lit r2))");
    assert_eq!(breadth_first(&task).plan.unwrap().cost, 2);
    let task = lights_problem("(imply (lit r1) (lit r3))");
    assert_eq!(breadth_first(&task).plan.unwrap().cost, 0);
    let task = lights_problem("(exists (?r - room) (lit ?r))");
    assert_eq!(breadth_first(&task).plan.unwrap().cost, 2);
    let task = lights_problem("(lit r3)");
    assert_eq!(breadth_first(&task).plan, None);
}
