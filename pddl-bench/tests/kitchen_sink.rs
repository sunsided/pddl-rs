//! The kitchen-sink domain of `pddl-codegen/tests` compiles and plans: its
//! ternary predicate, `either` and supertype parameters, numeric effects and
//! nullary derived predicate.

use pddl::{Parser, Problem};
use pddl_bench::domains::kitchen_sink;
use pddl_runtime::search::{breadth_first, replay};
use pddl_runtime::Task;

#[test]
fn rare_constructs_plan() {
    let problem = Problem::from_str(include_str!(
        "../../pddl-codegen/tests/fixtures/kitchen-sink-problem.pddl"
    ))
    .unwrap();
    let task = kitchen_sink::Task::new(&problem).unwrap();
    let initial = task.initial_state();
    assert!(initial.any_route, "the derived predicate holds initially");
    let plan = breadth_first(&task).plan.unwrap();
    let names: Vec<String> = plan.actions.iter().map(|&a| task.describe(a)).collect();
    // `?w` is the place `home`, which is the only object with `(type ?x)`.
    assert_eq!(names, ["(go t1 home shop home)"]);
    assert_eq!(replay(&task, &plan.actions), Some(1));
    let after = task.apply(&initial, plan.actions[0]);
    // The `when` reads the state before the action, where t1 isn't at shop yet,
    // so fuel stays 3. Load 1 is decreased, doubled and halved: 0.
    assert_eq!(after.fuel.get(&[0]), Some(3.0));
    assert_eq!(after.load.get(&[]), Some(0.0));
}
