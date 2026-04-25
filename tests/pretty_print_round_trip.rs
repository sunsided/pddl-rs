#![cfg(feature = "pretty")]

use pddl::{Domain, Parser, Pretty, Problem};

fn round_trip_domain(input: &str, width: usize, label: &str) {
    let (_, original) = Domain::parse(input).unwrap_or_else(|_| panic!("{label}: parse original"));
    let printed = original.pretty(width).to_string();
    let (_, reparsed) =
        Domain::parse(&*printed).unwrap_or_else(|_| panic!("{label}: parse printed output"));
    assert_eq!(
        original, reparsed,
        "{label}: AST mismatch at width {width}\n--- Printed ---\n{printed}"
    );
}

fn round_trip_problem(input: &str, width: usize, label: &str) {
    let (_, original) = Problem::parse(input).unwrap_or_else(|_| panic!("{label}: parse original"));
    let printed = original.pretty(width).to_string();
    let (_, reparsed) =
        Problem::parse(&*printed).unwrap_or_else(|_| panic!("{label}: parse printed output"));
    assert_eq!(
        original, reparsed,
        "{label}: AST mismatch at width {width}\n--- Printed ---\n{printed}"
    );
}

const STRIPS_DOMAIN: &str = r#"(define (domain blocks)
  (:requirements :strips :typing)
  (:types block)
  (:predicates (on ?x ?y - block) (clear ?x - block) (handempty) (holding ?x - block))
  (:action pickup
    :parameters (?x - block)
    :precondition (and (clear ?x) (handempty))
    :effect (and (not (clear ?x)) (not (handempty)) (holding ?x)))
  (:action putdown
    :parameters (?x - block)
    :precondition (holding ?x)
    :effect (and (clear ?x) (handempty) (not (holding ?x))))
)"#;

const STRIPS_PROBLEM: &str = r#"(define (problem blocks-prob)
  (:domain blocks)
  (:objects a b c - block)
  (:init (clear a) (clear b) (on a c) (on c place) (handempty))
  (:goal (and (on a b) (on b c)))
)"#;

const CONDITIONAL_EFFECTS_DOMAIN: &str = r#"(define (domain cond-eff)
  (:requirements :strips :typing :conditional-effects)
  (:types location physob)
  (:predicates (at ?x - physob ?y - location) (in ?x ?y - physob))
  (:action move
    :parameters (?x - physob ?from ?to - location)
    :precondition (at ?x ?from)
    :effect (and (at ?x ?to) (not (at ?x ?from))
      (forall (?z - physob)
        (when (in ?z ?x)
          (and (at ?z ?to) (not (at ?z ?from)))))))
)"#;

const DURATIVE_ACTIONS_DOMAIN: &str = r#"(define (domain durative)
  (:requirements :strips :typing :durative-actions)
  (:types location rover)
  (:predicates (at ?r - rover ?l - location))
  (:durative-action navigate
    :parameters (?r - rover ?from ?to - location)
    :duration (= ?duration 10)
    :condition (and (at start (at ?r ?from)))
    :effect (and (at end (at ?r ?to)) (at end (not (at ?r ?from)))))
)"#;

#[test]
fn strips_domain_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(STRIPS_DOMAIN, width, "strips-domain");
    }
}

#[test]
fn strips_problem_round_trip() {
    for width in [40, 80, 200] {
        round_trip_problem(STRIPS_PROBLEM, width, "strips-problem");
    }
}

#[test]
fn conditional_effects_domain_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(CONDITIONAL_EFFECTS_DOMAIN, width, "cond-eff-domain");
    }
}

#[test]
fn durative_actions_domain_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(DURATIVE_ACTIONS_DOMAIN, width, "durative-domain");
    }
}

#[test]
fn briefcase_world_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(BRIEFCASE_WORLD, width, "briefcase-domain");
        round_trip_problem(BRIEFCASE_WORLD_PROBLEM, width, "briefcase-problem");
    }
}

const BRIEFCASE_WORLD: &str = r#"
    (define (domain briefcase-world)
      (:requirements :strips :equality :typing :conditional-effects)
      (:types location physob)
      (:constants
            B ; the briefcase
            P ; the paycheck
            D
            - physob)
      (:predicates (at ?x - physob ?y - location)
                   (in ?x ?y - physob))

      ; Move briefcase from one location to another.
      (:action mov-B
           :parameters (?m ?l - location)
           :precondition (and (at B ?m) (not (= ?m ?l)))
           :effect (and (at B ?l) (not (at B ?m))
                        (forall (?z)
                            (when (and (in ?z) (not (= ?z B)))
                                  (and (at ?z ?l) (not (at ?z ?m)))))) )

      ; Put the item in the briefcase.
      (:action put-in
           :parameters (?x - physob ?l - location)
           :precondition (not (= ?x B))     ; the item must not be the briefcase itself
           :effect (when
                 (and (at ?x ?l) (at B ?l)) ; briefcase and item are at the same location
                 (in ?x)) )                 ; ... then the item is in the briefcase.

      ; Take the item out of the briefcase.
      (:action take-out
           :parameters (?x - physob)
           :precondition (not (= ?x B))     ; the item must be the briefcase itself
           :effect (not (in ?x)) )          ; the item is not in the briefcase anymore.
     )
     "#;

const BRIEFCASE_WORLD_PROBLEM: &str = r#"
    (define (problem get-paid)
        (:domain briefcase-world)
        (:init
               ; types: locations
               (place home) (place office)
               ; types: objects
               (object p) (object d) (object b)
               ; setup
               (at B home) (at P home) (at D home) (in P))
        (:goal (and (at B office) (at D office) (at P home)))
    )
    "#;
