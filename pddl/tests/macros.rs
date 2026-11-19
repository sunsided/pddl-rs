//! The macros through the `pddl` re-export, as users of the `macros` feature call them.

use std::sync::LazyLock;

use pddl::{Domain, PddlFile, Problem};

/// A domain checked at compile time through `pddl::domain!`.
static DOMAIN: LazyLock<Domain> = pddl::domain!("(define (domain d) (:predicates (p)))");

/// A domain from the corpus, through `pddl::domain_file!`.
static CORPUS: LazyLock<Domain> = pddl::domain_file!("../../tests/data/vhpop/blocks-domain.pddl");

/// A problem checked at compile time through `pddl::problem!`.
static PROBLEM: LazyLock<Problem> =
    pddl::problem!("(define (problem p) (:domain d) (:init) (:goal (p)))");

/// Both, through `pddl::pddl!`.
static BOTH: LazyLock<PddlFile> = pddl::pddl!(
    "(define (domain d) (:predicates (p))) (define (problem p) (:domain d) (:init) (:goal (p)))"
);

/// The checked source, through `pddl::validated!`.
const SOURCE: &str = pddl::validated!("(define (domain d))");

#[test]
fn re_exported_macros_expand() {
    assert_eq!(DOMAIN.name(), "d");
    assert_eq!(CORPUS.name(), "blocks");
    assert_eq!(PROBLEM.domain(), "d");
    assert_eq!((BOTH.domain_count(), BOTH.problem_count()), (1, 1));
    assert_eq!(SOURCE, "(define (domain d))");
}
