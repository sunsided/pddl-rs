//! Generated code compiles whatever the PDDL names, even where they match the
//! names of its own locals.

use pddl::{Parser, Problem};
use pddl_runtime::search::{astar, replay};

/// Parameters named like the generated locals (`state`, `text`, `v0`), and
/// bound types named like the loader's (`count`, `problem`, `state`).
#[pddl::task(
    domain_source = r"
(define (domain clash)
  (:requirements :typing :action-costs)
  (:types count problem state)
  (:predicates (seen ?a ?b - count) (done ?c - count) (marked ?s - state ?t - problem))
  (:functions (len ?a ?b - count) (total-cost))
  (:action step :parameters (?a ?v0 - count)
    :precondition (seen ?a ?v0)
    :effect (and (done ?v0) (increase (total-cost) (len ?a ?v0))))
  (:action mark :parameters (?state - state ?text - problem)
    :precondition (exists (?c - count) (done ?c))
    :effect (marked ?state ?text)))
",
    types(count = u32, problem = u32, state = u32)
)]
mod clash {}

#[test]
fn generated_locals_do_not_clash_with_pddl_names() {
    let problem = Problem::from_str(
        "(define (problem p) (:domain clash) (:objects c1 c2 - count x - problem s - state)
           (:init (seen c1 c2) (= (len c1 c2) 3) (= (total-cost) 0))
           (:goal (marked s x)) (:metric minimize (total-cost)))",
    )
    .unwrap();
    let task = clash::Task::new(&problem, |_| 1, |_| 2, |_| 3).unwrap();
    assert_eq!(task.bound().count, [1, 1]);
    let plan = astar(&task, |_| 0).plan.unwrap();
    let names: Vec<String> = plan.actions.iter().map(|&a| task.describe(a)).collect();
    assert_eq!(names, ["(step c1 c2)", "(mark s x)"]);
    assert_eq!(plan.cost, 3);
    assert_eq!(replay(&task, &plan.actions), Some(3));
}
