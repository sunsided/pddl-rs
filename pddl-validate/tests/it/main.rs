//! Integration tests of `pddl-validate` on hand-written domains, problems and plans.

mod classical;
mod model_errors;
mod mutation;
mod numeric;
mod pddl_plus;
mod plan_file;
mod temporal;
mod unsupported;

use pddl::{Domain, Parser, Problem};
use pddl_validate::{Report, ValidationStatus, Validator, Violation, ViolationKind};

/// A parsed domain and problem.
struct Task {
    /// The domain.
    domain: Domain,
    /// The problem.
    problem: Problem,
}

impl Task {
    /// Parses a domain and a problem; panics if either does not parse.
    fn new(domain: &str, problem: &str) -> Self {
        Self {
            domain: Domain::from_str(domain).expect("the domain parses"),
            problem: Problem::from_str(problem).expect("the problem parses"),
        }
    }

    /// Builds the validator; panics on a model error.
    fn validator(&self) -> Validator<'_> {
        Validator::new(&self.domain, &self.problem).expect("the task is well-formed")
    }

    /// Validates a plan file.
    fn run(&self, plan: &str) -> Report {
        self.validator().validate_text(plan)
    }

    /// Validates a plan file that must be invalid and returns its violation.
    fn violation(&self, plan: &str) -> Violation {
        let report = self.run(plan);
        assert_eq!(report.status(), ValidationStatus::Invalid, "{report:?}");
        report
            .violation()
            .cloned()
            .expect("an invalid report has a violation")
    }

    /// Validates a plan file that must fail at step `number` and returns the failure.
    fn step_failure(&self, plan: &str, number: usize) -> ViolationKind {
        let violation = self.violation(plan);
        let step = violation.step.as_ref().expect("a step fails");
        assert_eq!(step.number, number, "{violation}");
        violation.kind
    }
}
