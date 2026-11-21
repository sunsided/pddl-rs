//! The `pddl/validatePlan` request: checks a plan against a domain and problem with
//! [`pddl_validate`] and answers with a verdict a client can show as a diagnostic.
//!
//! The request carries the three texts, not file names, so it works for unsaved
//! documents and in the WebAssembly build, which cannot read arbitrary files.

use pddl::{Domain, Optimization, Parser as _, Problem};
use pddl_validate::{Validator, ViolationKind};
use serde::{Deserialize, Serialize};

/// The method name of the request.
pub const METHOD: &str = "pddl/validatePlan";

/// The parameters of a `pddl/validatePlan` request.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ValidatePlanParams {
    /// The text of the domain.
    pub domain: String,
    /// The text of the problem, whose `(:domain ...)` should name `domain`.
    pub problem: String,
    /// The text of the plan, as planners write plan files.
    pub plan: String,
}

/// The answer to a `pddl/validatePlan` request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlanVerdict {
    /// `valid`, `invalid`, `unsupported`, or `error` when the domain or problem cannot
    /// be read or checked.
    pub status: Status,
    /// What the validator found, in one line; empty for a valid plan.
    pub message: String,
    /// The 0-based line of the plan the message belongs to; `None` for the goal, the
    /// task as a whole, or a valid plan.
    pub line: Option<u32>,
    /// A VAL-style repair hint for an invalid plan.
    pub advice: Option<String>,
    /// The value of the problem's metric in the final state of a valid plan, if it has
    /// one and it is defined.
    pub metric: Option<f64>,
    /// `true` if the metric is minimized, `false` if maximized; `None` without a metric.
    pub minimize: Option<bool>,
}

/// The verdict kinds of a [`PlanVerdict`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    /// The plan solves the problem.
    Valid,
    /// The plan does not solve the problem; see the message.
    Invalid,
    /// The task uses a construct the validator does not interpret, so it gives no verdict.
    Unsupported,
    /// The domain or problem cannot be read, or they do not fit together.
    Error,
}

/// Validates the plan of `params` against its domain and problem.
#[must_use]
pub fn validate(params: &ValidatePlanParams) -> PlanVerdict {
    let domain = match Domain::from_str(&params.domain) {
        Ok(domain) => domain,
        Err(error) => return error_verdict(format!("cannot read the domain: {error}")),
    };
    let problem = match Problem::from_str(&params.problem) {
        Ok(problem) => problem,
        Err(error) => return error_verdict(format!("cannot read the problem: {error}")),
    };
    let validator = match Validator::new(&domain, &problem) {
        Ok(validator) => validator,
        Err(error) => return error_verdict(error.to_string()),
    };

    verdict(&validator.validate_text(&params.plan))
}

/// Turns a validation report into the verdict a client shows.
fn verdict(report: &pddl_validate::Report) -> PlanVerdict {
    let mut verdict = PlanVerdict {
        status: Status::Valid,
        message: String::new(),
        line: None,
        advice: None,
        metric: None,
        minimize: None,
    };
    // An invalid report has a violation and an unsupported one its construct.
    if let Some(violation) = report.violation() {
        verdict.status = Status::Invalid;
        verdict.message = violation.to_string();
        verdict.advice = Some(violation.advice());
        verdict.line = violation_line(violation);
    } else if let Some(unsupported) = report.unsupported() {
        verdict.status = Status::Unsupported;
        verdict.message = unsupported.to_string();
    } else if let Some(metric) = report.metric() {
        verdict.metric = Some(metric.value);
        verdict.minimize = Some(metric.optimization == Optimization::Minimize);
    }

    verdict
}

/// The 0-based plan line a violation belongs to, if it names one.
fn violation_line(violation: &pddl_validate::Violation) -> Option<u32> {
    let line = match &violation.kind {
        ViolationKind::PlanSyntax(error) => error.line,
        _ => violation.step.as_ref()?.line,
    };

    // Lines count from 1; 0 means a plan built in code, which has no file.
    line.checked_sub(1)
        .and_then(|line| u32::try_from(line).ok())
}

/// A verdict for a task the validator cannot check.
fn error_verdict(message: String) -> PlanVerdict {
    PlanVerdict {
        status: Status::Error,
        message,
        line: None,
        advice: None,
        metric: None,
        minimize: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A domain where `(flip)` toggles `(on)`.
    const DOMAIN: &str = "(define (domain d) (:requirements :negative-preconditions :action-costs)
        (:predicates (on)) (:functions (total-cost))
        (:action flip :parameters () :precondition (not (on))
          :effect (and (on) (increase (total-cost) 2))))";

    /// A problem for [`DOMAIN`] that wants `(on)` at the lowest cost.
    const PROBLEM: &str = "(define (problem p) (:domain d)
        (:init (= (total-cost) 0)) (:goal (on)) (:metric minimize (total-cost)))";

    /// Validates `plan` against [`DOMAIN`] and [`PROBLEM`].
    fn check(plan: &str) -> PlanVerdict {
        validate(&ValidatePlanParams {
            domain: DOMAIN.into(),
            problem: PROBLEM.into(),
            plan: plan.into(),
        })
    }

    #[test]
    fn a_valid_plan_reports_its_metric() {
        let verdict = check("(flip)\n; cost = 2 (general cost)\n");
        assert_eq!(verdict.status, Status::Valid);
        assert_eq!((verdict.metric, verdict.minimize), (Some(2.0), Some(true)));
        assert_eq!(verdict.line, None);
    }

    #[test]
    fn an_invalid_step_names_its_line() {
        let verdict = check("; first\n(flip)\n(flip)\n");
        assert_eq!(verdict.status, Status::Invalid);
        assert_eq!(verdict.line, Some(2));
        assert_eq!(
            verdict.message,
            "step 2 (flip): unsatisfied precondition: (not (on))"
        );
        assert!(verdict.advice.is_some());
    }

    #[test]
    fn an_unmet_goal_has_no_line() {
        let verdict = check("");
        assert_eq!(verdict.status, Status::Invalid);
        assert_eq!(verdict.line, None);
        assert!(verdict.message.contains("(on)"), "{}", verdict.message);
    }

    #[test]
    fn a_plan_syntax_error_names_its_line() {
        let verdict = check("(flip)\n(flip");
        assert_eq!(verdict.status, Status::Invalid);
        assert_eq!(verdict.line, Some(1));
    }

    #[test]
    fn unreadable_or_mismatched_tasks_are_errors() {
        let verdict = validate(&ValidatePlanParams {
            domain: "(define (domain".into(),
            problem: PROBLEM.into(),
            plan: String::new(),
        });
        assert_eq!(verdict.status, Status::Error);
        assert!(
            verdict.message.starts_with("cannot read the domain"),
            "{}",
            verdict.message
        );

        let verdict = validate(&ValidatePlanParams {
            domain: DOMAIN.into(),
            problem: "(define (problem".into(),
            plan: String::new(),
        });
        assert_eq!(verdict.status, Status::Error);
        assert!(
            verdict.message.starts_with("cannot read the problem"),
            "{}",
            verdict.message
        );

        let verdict = validate(&ValidatePlanParams {
            domain: DOMAIN.into(),
            problem: "(define (problem p) (:domain d) (:init) (:goal (gone)))".into(),
            plan: String::new(),
        });
        assert_eq!(verdict.status, Status::Error);
    }

    #[test]
    fn unsupported_constructs_give_no_verdict() {
        let verdict = validate(&ValidatePlanParams {
            domain: DOMAIN.into(),
            problem: "(define (problem p) (:domain d) (:init)
                (:goal (preference p (on))))"
                .into(),
            plan: "(flip)".into(),
        });
        assert_eq!(verdict.status, Status::Unsupported);
        assert_ne!(verdict.message, "");
    }
}
