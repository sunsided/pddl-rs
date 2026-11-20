//! Generated code with derived predicates (#130).

use pddl::{Parser, Problem};
use pddl_bench::domains::wiring;
use pddl_bench::fixture;
use pddl_runtime::search::{astar, ida_star, replay};
use pddl_runtime::Task;

#[test]
fn recursive_derived_predicates_reach_the_known_optimum() {
    // Fast Downward finds 4: plant -> junction -> lamp, or plant -> shed costs 5.
    let (_, problem) = fixture("wiring");
    let task = wiring::Task::new(&problem).unwrap();
    for plan in [astar(&task, |_| 0).plan, ida_star(&task, |_| 0).plan] {
        let plan = plan.unwrap();
        assert_eq!(plan.cost, 4);
        assert_eq!(replay(&task, &plan.actions), Some(4));
    }
}

#[test]
fn derived_facts_follow_the_basic_facts() {
    let problem = Problem::from_str(
        "(define (problem p) (:domain wiring) (:objects a b c - node)
           (:init (source a) (duct a b) (duct b c) (= (duct-length a b) 1) (= (duct-length b c) 1))
           (:goal (powered c)))",
    )
    .unwrap();
    let task = wiring::Task::new(&problem).unwrap();
    let initial = task.initial_state();
    // Only the source is powered at first, so a cable may go into b, not into a.
    let powered = |s: &wiring::State| {
        s.powered
            .iter()
            .map(|n| task.names()[n.0 as usize].clone())
            .collect::<Vec<_>>()
    };
    assert_eq!(powered(&initial), ["a"]);
    let mut out = Vec::new();
    task.successors(&initial, &mut out);
    let names: Vec<String> = out.iter().map(|(a, _)| task.describe(*a)).collect();
    assert_eq!(names, ["(lay-cable a b)", "(lay-cable b c)"]);
    let (_, after) = &out[0];
    assert_eq!(powered(after), ["a", "b"]);
    // Laying b -> c first powers nothing new: b is dead.
    let (_, dead) = &out[1];
    assert_eq!(powered(dead), ["a"]);
}
