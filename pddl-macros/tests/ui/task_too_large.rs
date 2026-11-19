#[pddl_macros::task(
    domain_source = r#"(define (domain d) (:predicates (p ?x))
      (:action a :parameters (?x) :precondition () :effect (p ?x)))"#,
    problem_source = r#"(define (problem q) (:domain d) (:objects o1 o2 o3) (:init) (:goal (p o1)))"#,
    max_operators = 2,
)]
mod d {}

fn main() {}
