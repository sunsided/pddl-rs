#![cfg(feature = "pretty")]

use pddl::{Domain, Parser, Pretty, Problem};

fn round_trip_domain(input: &str, width: usize, label: &str) {
    let (_, original) = Domain::parse(input).unwrap_or_else(|_| panic!("{label}: parse original"));
    let printed = original.pretty(width).to_string();
    let (_, reparsed) =
        Domain::parse(&*printed).unwrap_or_else(|e| panic!("{label}: parse printed output at width {width}\n--- Printed ---\n{printed}\n--- Error ---\n{e:?}"));
    assert_eq!(
        original, reparsed,
        "{label}: AST mismatch at width {width}\n--- Printed ---\n{printed}"
    );
}

fn round_trip_problem(input: &str, width: usize, label: &str) {
    let (_, original) = Problem::parse(input).unwrap_or_else(|_| panic!("{label}: parse original"));
    let printed = original.pretty(width).to_string();
    let (_, reparsed) =
        Problem::parse(&*printed).unwrap_or_else(|e| panic!("{label}: parse printed output at width {width}\n--- Printed ---\n{printed}\n--- Error ---\n{e:?}"));
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

const METRIC_DOMAIN: &str = r#"(define (domain cost-domain)
  (:requirements :strips :typing :numeric-fluents :action-costs)
  (:types location)
  (:predicates (at ?l - location))
  (:functions (total-cost))
  (:action move
    :parameters (?from ?to - location)
    :precondition (at ?from)
    :effect (and (at ?to) (not (at ?from)) (increase (total-cost) 1)))
)"#;

const METRIC_PROBLEM: &str = r#"(define (problem cost-problem)
  (:domain cost-domain)
  (:objects l1 l2 l3 - location)
  (:init (at l1))
  (:goal (at l3))
  (:metric minimize (total-cost))
)"#;

const LENGTH_DOMAIN: &str = r#"(define (domain blocks)
  (:requirements :strips :typing)
  (:types block)
  (:predicates (on ?x ?y - block) (clear ?x - block))
  (:action stack
    :parameters (?x ?y - block)
    :precondition (and (clear ?x) (clear ?y))
    :effect (and (on ?x ?y) (not (clear ?x))))
)"#;

const LENGTH_PROBLEM: &str = r#"(define (problem length-problem)
  (:domain blocks)
  (:objects a b c - block)
  (:init (clear a) (on a b))
  (:goal (and (on b a)))
  (:length (:serial 10) (:parallel 5))
)"#;

const CONSTRAINTS_DOMAIN: &str = r#"(define (domain constraints-domain)
  (:requirements :strips :typing :constraints)
  (:types block)
  (:predicates (on ?x ?y - block) (clear ?x - block))
  (:constraints (and))
  (:action stack
    :parameters (?x ?y - block)
    :precondition (and (clear ?x) (clear ?y))
    :effect (and (on ?x ?y) (not (clear ?x))))
)"#;

const CONSTRAINTS_PROBLEM: &str = r#"(define (problem constraints-problem)
  (:domain constraints-domain)
  (:objects a b c - block)
  (:init (clear a) (clear b) (clear c))
  (:goal (and (on a b)))
  (:constraints (and))
)"#;

const DERIVED_DOMAIN: &str = r#"(define (domain derived-domain)
  (:requirements :strips :typing :derived-predicates)
  (:types block)
  (:predicates (on ?x ?y - block) (clear ?x - block))
  (:derived (above ?x ?y - block)
    (or (on ?x ?y) (exists (?z - block) (and (on ?x ?z) (above ?z ?y)))))
  (:action stack
    :parameters (?x ?y - block)
    :precondition (and (clear ?x) (clear ?y))
    :effect (and (on ?x ?y) (not (clear ?x))))
)"#;

const NUMERIC_EFFECTS_DOMAIN: &str = r#"(define (domain numeric-domain)
  (:requirements :strips :typing :numeric-fluents)
  (:types robot)
  (:predicates (charged ?r - robot))
  (:functions (battery-level ?r - robot) (max-charge ?r - robot))
  (:action charge
    :parameters (?r - robot)
    :precondition (not (charged ?r))
    :effect (and (charged ?r) (assign (battery-level ?r) (max-charge ?r))))
  (:action drain
    :parameters (?r - robot)
    :precondition (charged ?r)
    :effect (and (not (charged ?r)) (decrease (battery-level ?r) 5)))
)"#;

const PREFERENCE_DOMAIN: &str = r#"(define (domain blocks)
  (:requirements :strips :typing :preferences)
  (:types block)
  (:predicates (on ?x ?y - block) (clear ?x - block))
  (:action stack
    :parameters (?x ?y - block)
    :precondition (and (clear ?x) (clear ?y))
    :effect (and (on ?x ?y) (not (clear ?x))))
)"#;

const PREFERENCE_PROBLEM: &str = r#"(define (problem preference-problem)
  (:domain blocks)
  (:objects a b c - block)
  (:init (clear a) (clear b) (on a c))
  (:goal (and (preference goal1 (on a b)) (preference goal2 (on b c))))
)"#;

const TIMELESS_DOMAIN: &str = r#"(define (domain timeless-domain)
  (:requirements :strips :equality)
  (:predicates (p ?x) (q ?x ?y))
  (:timeless (= x y) (p a))
  (:action noop
    :parameters ()
    :precondition (and)
    :effect (and))
)"#;

#[test]
fn metric_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(METRIC_DOMAIN, width, "metric-domain");
        round_trip_problem(METRIC_PROBLEM, width, "metric-problem");
    }
}

#[test]
fn length_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(LENGTH_DOMAIN, width, "length-domain");
        round_trip_problem(LENGTH_PROBLEM, width, "length-problem");
    }
}

#[test]
fn constraints_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(CONSTRAINTS_DOMAIN, width, "constraints-domain");
        round_trip_problem(CONSTRAINTS_PROBLEM, width, "constraints-problem");
    }
}

#[test]
fn derived_predicates_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(DERIVED_DOMAIN, width, "derived-domain");
    }
}

#[test]
fn numeric_effects_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(NUMERIC_EFFECTS_DOMAIN, width, "numeric-effects-domain");
    }
}

#[test]
fn preference_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(PREFERENCE_DOMAIN, width, "preference-domain");
        round_trip_problem(PREFERENCE_PROBLEM, width, "preference-problem");
    }
}

#[test]
fn timeless_round_trip() {
    for width in [40, 80, 200] {
        round_trip_domain(TIMELESS_DOMAIN, width, "timeless-domain");
    }
}

const ALL_DOMAINS: &[(&str, &str)] = &[
    ("strips", STRIPS_DOMAIN),
    ("conditional-effects", CONDITIONAL_EFFECTS_DOMAIN),
    ("durative-actions", DURATIVE_ACTIONS_DOMAIN),
    ("briefcase-world", BRIEFCASE_WORLD),
    ("metric", METRIC_DOMAIN),
    ("length", LENGTH_DOMAIN),
    ("constraints", CONSTRAINTS_DOMAIN),
    ("derived", DERIVED_DOMAIN),
    ("numeric-effects", NUMERIC_EFFECTS_DOMAIN),
    ("preference", PREFERENCE_DOMAIN),
    ("timeless", TIMELESS_DOMAIN),
];

const ALL_PROBLEMS: &[(&str, &str)] = &[
    ("strips", STRIPS_PROBLEM),
    ("briefcase-world", BRIEFCASE_WORLD_PROBLEM),
    ("metric", METRIC_PROBLEM),
    ("length", LENGTH_PROBLEM),
    ("constraints", CONSTRAINTS_PROBLEM),
    ("preference", PREFERENCE_PROBLEM),
];

/// PDDL is case-insensitive: an all-uppercase file must parse to an AST equal to the lowercase one.
#[test]
fn uppercase_input_parses_to_the_same_ast() {
    for (label, input) in ALL_DOMAINS {
        let lower = Domain::from_str(input).unwrap_or_else(|e| panic!("{label}: {e}"));
        let upper = Domain::from_str(&input.to_ascii_uppercase())
            .unwrap_or_else(|e| panic!("{label} (uppercase): {e}"));
        assert_eq!(lower, upper, "{label}");
    }
    for (label, input) in ALL_PROBLEMS {
        let lower = Problem::from_str(input).unwrap_or_else(|e| panic!("{label}: {e}"));
        let upper = Problem::from_str(&input.to_ascii_uppercase())
            .unwrap_or_else(|e| panic!("{label} (uppercase): {e}"));
        assert_eq!(lower, upper, "{label}");
    }
}

/// Names keep the spelling they were written with; keywords are printed in lowercase.
#[test]
fn mixed_case_names_keep_their_spelling_when_printed() {
    let domain = Domain::from_str(
        "(DEFINE (DOMAIN Blocks) (:REQUIREMENTS :STRIPS) (:PREDICATES (On ?X ?Y)))",
    )
    .unwrap();
    assert_eq!(domain.name(), "blocks");
    assert_eq!(domain.name().to_string(), "Blocks");
    let printed = domain.pretty(80).to_string();
    assert!(printed.contains("(domain Blocks)"), "{printed}");
    assert!(printed.contains("(:requirements :strips)"), "{printed}");
    assert!(printed.contains("(On ?X ?Y)"), "{printed}");
}

/// Replaces every run of whitespace directly before a `(` with `separator`
/// (after removing `;` comments, so no form ends up inside a comment).
fn respace_before_parens(input: &str, separator: &str) -> String {
    let uncommented: String = input
        .lines()
        .map(|line| line.split(';').next().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    let mut out = String::with_capacity(uncommented.len());
    let mut pending = String::new();
    for c in uncommented.chars() {
        if c.is_whitespace() {
            pending.push(c);
            continue;
        }
        if c == '(' && !pending.is_empty() {
            out.push_str(separator);
        } else {
            out.push_str(&pending);
        }
        pending.clear();
        out.push(c);
    }
    out.push_str(&pending);
    out
}

/// Whitespace before a `(` is optional, and a `;` comment can stand in for it.
#[test]
fn whitespace_before_parens_is_optional() {
    for separator in ["", " ; comment\n"] {
        for (label, input) in ALL_DOMAINS {
            let expected = Domain::from_str(input).unwrap();
            let respaced = respace_before_parens(input, separator);
            let actual = Domain::from_str(&respaced)
                .unwrap_or_else(|e| panic!("{label} {separator:?}: {e}\n{respaced}"));
            assert_eq!(expected, actual, "{label} {separator:?}");
        }
        for (label, input) in ALL_PROBLEMS {
            let expected = Problem::from_str(input).unwrap();
            let respaced = respace_before_parens(input, separator);
            let actual = Problem::from_str(&respaced)
                .unwrap_or_else(|e| panic!("{label} {separator:?}: {e}\n{respaced}"));
            assert_eq!(expected, actual, "{label} {separator:?}");
        }
    }
}
