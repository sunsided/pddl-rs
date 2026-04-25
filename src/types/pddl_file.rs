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
