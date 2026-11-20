//! Validation throughput on generated Gripper tasks with action costs.
//!
//! Save a baseline with `cargo bench -p pddl-validate -- --save-baseline <name>` and
//! compare a later change against it with `-- --baseline <name>`.

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use pddl::{Domain, Parser, Problem};
use pddl_validate::{Plan, ValidationStatus, Validator};
use std::fmt::Write;
use std::hint::black_box;

/// Gripper with typing, action costs and a quantified precondition on `move` that
/// always holds but makes every move look at every ball.
const DOMAIN: &str = "
(define (domain gripper) (:requirements :typing :universal-preconditions :action-costs)
  (:types room ball gripper)
  (:predicates (at-robby ?r - room) (at ?b - ball ?r - room)
               (free ?g - gripper) (carry ?b - ball ?g - gripper))
  (:functions (total-cost) - number)
  (:action move :parameters (?from ?to - room)
    :precondition (and (at-robby ?from)
                       (forall (?g - gripper) (or (free ?g) (exists (?o - ball) (carry ?o ?g)))))
    :effect (and (at-robby ?to) (not (at-robby ?from)) (increase (total-cost) 1)))
  (:action pick :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (at ?b ?r) (at-robby ?r) (free ?g))
    :effect (and (carry ?b ?g) (not (at ?b ?r)) (not (free ?g)) (increase (total-cost) 1)))
  (:action drop :parameters (?b - ball ?r - room ?g - gripper)
    :precondition (and (carry ?b ?g) (at-robby ?r))
    :effect (and (at ?b ?r) (free ?g) (not (carry ?b ?g)) (increase (total-cost) 1))))";

/// A problem with `balls` balls in room a, all wanted in room b, and its plan.
fn task(balls: usize) -> (String, String) {
    let mut objects = String::new();
    let mut init = String::new();
    let mut goal = String::new();
    let mut plan = String::new();
    for i in 0..balls {
        write!(objects, " b{i}").unwrap();
        write!(init, " (at b{i} a)").unwrap();
        write!(goal, " (at b{i} b)").unwrap();
    }

    for i in 0..balls {
        writeln!(
            plan,
            "(pick b{i} a left) (move a b) (drop b{i} b left) (move b a)"
        )
        .unwrap();
    }

    let problem = format!(
        "(define (problem p) (:domain gripper)
           (:objects a b - room left - gripper{objects} - ball)
           (:init (at-robby a) (free left){init} (= (total-cost) 0))
           (:goal (and{goal})) (:metric minimize (total-cost)))"
    );
    (problem, plan)
}

/// Benchmarks validating Gripper plans of growing length.
fn validate(c: &mut Criterion) {
    let domain = Domain::from_str(DOMAIN).unwrap();
    let mut group = c.benchmark_group("validate");
    for balls in [10, 40] {
        let (problem, plan) = task(balls);
        let problem = Problem::from_str(&problem).unwrap();
        let plan: Plan = plan.parse().unwrap();
        let validator = Validator::new(&domain, &problem).unwrap();
        assert_eq!(validator.validate(&plan).status(), ValidationStatus::Valid);

        group.throughput(Throughput::Elements(plan.len() as u64));
        group.bench_function(format!("gripper-{balls}"), |b| {
            b.iter(|| validator.validate(black_box(&plan)));
        });
    }

    group.finish();
}

criterion_group!(benches, validate);
criterion_main!(benches);
