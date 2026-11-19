//! Calls every macro from a real crate and compares the result with parsing at
//! run time.

use std::sync::LazyLock;

use pddl::{Domain, Parser, PddlFile, Problem};

/// A domain from the IPC-era VHPOP corpus in the `pddl` crate's test data.
static CORPUS_DOMAIN: LazyLock<Domain> =
    pddl_macros::domain_file!("../../tests/data/vhpop/blocks-domain.pddl");

/// A problem from the same corpus.
static CORPUS_PROBLEM: LazyLock<Problem> =
    pddl_macros::problem_file!("../../tests/data/vhpop/bw-large-a.pddl");

/// The test domain, from a file next to this one.
static SWITCHES: LazyLock<Domain> = pddl_macros::domain_file!("fixtures/switches-domain.pddl");

/// The test problem, from a file next to this one.
static TWO_SWITCHES: LazyLock<Problem> =
    pddl_macros::problem_file!("fixtures/switches-problem.pddl");

/// The test domain and problem as one file.
static BOTH: LazyLock<PddlFile> = pddl_macros::pddl!(
    "
    (define (domain switches)
      (:requirements :strips :negative-preconditions)
      (:predicates (on ?s))
      (:action flip :parameters (?s) :precondition (not (on ?s)) :effect (on ?s)))
    (define (problem two-switches) (:domain switches)
      (:objects s1 s2) (:init) (:goal (and (on s1) (on s2))))
    "
);

/// The checked source of the test domain.
const SWITCHES_SOURCE: &str = pddl_macros::validated_file!("fixtures/switches-domain.pddl");

/// The checked source of an inline domain.
const INLINE_SOURCE: &str = pddl_macros::validated!("(define (domain d))");

#[test]
fn file_macros_match_runtime_parsing() {
    let read = |path: &str| {
        std::fs::read_to_string(format!("{}/tests/{path}", env!("CARGO_MANIFEST_DIR"))).unwrap()
    };
    assert_eq!(
        *CORPUS_DOMAIN,
        Domain::from_str(&read("../../tests/data/vhpop/blocks-domain.pddl")).unwrap()
    );
    assert_eq!(
        *CORPUS_PROBLEM,
        Problem::from_str(&read("../../tests/data/vhpop/bw-large-a.pddl")).unwrap()
    );
    assert_eq!(
        *SWITCHES,
        Domain::from_str(&read("fixtures/switches-domain.pddl")).unwrap()
    );
    assert_eq!(TWO_SWITCHES.domain(), "switches");
    assert_eq!(SWITCHES_SOURCE, read("fixtures/switches-domain.pddl"));
}

#[test]
fn inline_macros() {
    let domain: &Domain = &pddl_macros::domain!("(define (domain d) (:predicates (p) (q)))");
    assert_eq!(domain.predicates().len(), 2);

    let problem: &Problem =
        &pddl_macros::problem!("(define (problem p) (:domain d) (:init) (:goal (and)))",);
    assert_eq!(problem.name(), "p");

    assert_eq!((BOTH.domain_count(), BOTH.problem_count()), (1, 1));
    assert_eq!(BOTH.domains[0], *SWITCHES);

    assert_eq!(INLINE_SOURCE, "(define (domain d))");
}
