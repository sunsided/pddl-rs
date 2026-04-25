//! Defines the [`PddlFile`] type for multi-definition PDDL files.

use crate::{Domain, Problem};

/// A PDDL file containing zero or more domains and problems.
///
/// PDDL files can contain multiple `(define (domain ...))` and `(define (problem ...))`
/// blocks in any order. This struct collects them all.
#[derive(Debug, Clone, Default)]
pub struct PddlFile {
    /// All domains found in the file.
    pub domains: Vec<Domain>,
    /// All problems found in the file.
    pub problems: Vec<Problem>,
}

impl PddlFile {
    /// Creates a new empty [`PddlFile`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a [`PddlFile`] with the given domains and problems.
    pub fn with_domains_and_problems(domains: Vec<Domain>, problems: Vec<Problem>) -> Self {
        Self { domains, problems }
    }

    /// Adds a domain to this file.
    pub fn with_domain(mut self, domain: Domain) -> Self {
        self.domains.push(domain);
        self
    }

    /// Adds a problem to this file.
    pub fn with_problem(mut self, problem: Problem) -> Self {
        self.problems.push(problem);
        self
    }

    /// Returns `true` if this file contains no domains and no problems.
    pub fn is_empty(&self) -> bool {
        self.domains.is_empty() && self.problems.is_empty()
    }

    /// Returns the number of domains in this file.
    pub fn domain_count(&self) -> usize {
        self.domains.len()
    }

    /// Returns the number of problems in this file.
    pub fn problem_count(&self) -> usize {
        self.problems.len()
    }
}

#[cfg(all(test, feature = "parser"))]
mod tests {
    use crate::parsers::Parser;
    use crate::{Domain, PddlFile, Problem};

    fn sample_domain() -> Domain {
        Domain::from_str("(define (domain test) (:requirements :strips) (:predicates (p)))")
            .unwrap()
    }

    fn sample_problem() -> Problem {
        Problem::from_str("(define (problem p1) (:domain test) (:init) (:goal (and)))").unwrap()
    }

    #[test]
    fn test_new_creates_empty_file() {
        let file = PddlFile::new();
        assert!(file.is_empty());
        assert_eq!(file.domain_count(), 0);
        assert_eq!(file.problem_count(), 0);
    }

    #[test]
    fn test_default_creates_empty_file() {
        let file = PddlFile::default();
        assert!(file.is_empty());
        assert_eq!(file.domain_count(), 0);
        assert_eq!(file.problem_count(), 0);
    }

    #[test]
    fn test_with_domains_and_problems() {
        let domain = sample_domain();
        let problem = sample_problem();
        let file = PddlFile::with_domains_and_problems(vec![domain.clone()], vec![problem.clone()]);
        assert_eq!(file.domain_count(), 1);
        assert_eq!(file.problem_count(), 1);
        assert!(!file.is_empty());
    }

    #[test]
    fn test_with_domain_builder() {
        let file = PddlFile::new()
            .with_domain(sample_domain())
            .with_domain(sample_domain());
        assert_eq!(file.domain_count(), 2);
        assert!(!file.is_empty());
    }

    #[test]
    fn test_with_problem_builder() {
        let file = PddlFile::new()
            .with_problem(sample_problem())
            .with_problem(sample_problem());
        assert_eq!(file.problem_count(), 2);
        assert!(!file.is_empty());
    }

    #[test]
    fn test_with_domain_and_problem_chained() {
        let file = PddlFile::new()
            .with_domain(sample_domain())
            .with_problem(sample_problem())
            .with_domain(sample_domain());
        assert_eq!(file.domain_count(), 2);
        assert_eq!(file.problem_count(), 1);
        assert!(!file.is_empty());
    }

    #[test]
    fn test_clone() {
        let original = PddlFile::new()
            .with_domain(sample_domain())
            .with_problem(sample_problem());
        let cloned = original.clone();
        assert_eq!(cloned.domain_count(), original.domain_count());
        assert_eq!(cloned.problem_count(), original.problem_count());
        assert_eq!(original.domain_count(), 1);
        assert_eq!(original.problem_count(), 1);
    }

    #[test]
    fn test_debug_format() {
        let file = PddlFile::new();
        let debug = format!("{:?}", file);
        assert!(debug.contains("PddlFile"));
    }
}
