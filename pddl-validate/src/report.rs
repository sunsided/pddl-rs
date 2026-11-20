//! The result of validating a plan: verdict, diagnostic and execution trace.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

use crate::atom::Atom;
use crate::ValidationStatus;

/// What [`Validator::validate`](crate::Validator::validate) found.
#[derive(Debug, Clone, PartialEq)]
pub struct Report {
    /// The verdict, with its diagnostic.
    outcome: Outcome,
    /// What each executed step changed, in order.
    trace: Vec<StepTrace>,
    /// The value of the problem's metric in the final state of a valid plan; `None` if the
    /// problem has none or its value is undefined.
    metric: Option<Metric>,
}

impl Report {
    /// Builds a report without a metric value.
    pub(crate) const fn new(outcome: Outcome, trace: Vec<StepTrace>) -> Self {
        Self {
            outcome,
            trace,
            metric: None,
        }
    }

    /// Adds the metric value.
    pub(crate) const fn with_metric(mut self, metric: Metric) -> Self {
        self.metric = Some(metric);
        self
    }

    /// Returns the value of the problem's `:metric` in the final state, for a valid plan
    /// of a problem that has one.
    ///
    /// `None` if the plan is not valid, if the problem has no metric, or if its value is
    /// undefined in the final state (it reads a fluent without a value, divides by zero
    /// or leaves the finite range); an undefined metric does not make the plan invalid.
    ///
    /// # Examples
    ///
    /// ```
    /// use pddl::{Domain, Optimization, Parser, Problem};
    /// use pddl_validate::Validator;
    ///
    /// let domain = Domain::from_str("(define (domain d) (:requirements :action-costs)
    ///     (:predicates (done)) (:functions (total-cost) - number)
    ///     (:action work :parameters () :effect (and (done) (increase (total-cost) 2.5))))").unwrap();
    /// let problem = Problem::from_str("(define (problem p) (:domain d)
    ///     (:init (= (total-cost) 0)) (:goal (done)) (:metric minimize (total-cost)))").unwrap();
    /// let report = Validator::new(&domain, &problem).unwrap().validate_text("(work) (work)");
    /// let metric = report.metric().unwrap();
    /// assert_eq!((metric.optimization, metric.value), (Optimization::Minimize, 5.0));
    /// ```
    #[must_use]
    pub const fn metric(&self) -> Option<&Metric> {
        self.metric.as_ref()
    }

    /// Returns the verdict.
    #[must_use]
    pub const fn status(&self) -> ValidationStatus {
        match self.outcome {
            Outcome::Valid => ValidationStatus::Valid,
            Outcome::Invalid(_) => ValidationStatus::Invalid,
            Outcome::Unsupported(_) => ValidationStatus::Unsupported,
        }
    }

    /// Returns the verdict with its diagnostic.
    #[must_use]
    pub const fn outcome(&self) -> &Outcome {
        &self.outcome
    }

    /// Returns why the plan is invalid, or `None` if it is not.
    #[must_use]
    pub const fn violation(&self) -> Option<&Violation> {
        match &self.outcome {
            Outcome::Invalid(violation) => Some(violation),
            _ => None,
        }
    }

    /// Returns the construct the validator cannot interpret, or `None` if there is none.
    #[must_use]
    pub const fn unsupported(&self) -> Option<&Unsupported> {
        match &self.outcome {
            Outcome::Unsupported(unsupported) => Some(unsupported),
            _ => None,
        }
    }

    /// Returns what each step changed, for the steps that were applied.
    ///
    /// A valid sequential plan has one entry per step, a valid temporal plan one per
    /// happening. An invalid plan has entries up to the failing step or happening. An
    /// unsupported one has none.
    #[must_use]
    pub fn trace(&self) -> &[StepTrace] {
        &self.trace
    }
}

/// The verdict of a [`Report`], with its diagnostic.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Outcome {
    /// Every step is applicable and the final state satisfies the goal.
    Valid,
    /// The first violation found. Validation stops there, as VAL's does.
    Invalid(Violation),
    /// A construct the validator does not interpret. The plan is neither accepted nor
    /// rejected.
    Unsupported(Unsupported),
}

/// Why a plan is invalid.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct Violation {
    /// The step that fails, or `None` if the plan text or the goal does.
    pub step: Option<StepRef>,
    /// What fails.
    pub kind: ViolationKind,
}

impl Violation {
    /// Returns a repair hint in the style of VAL's "Plan Repair Advice".
    ///
    /// # Examples
    ///
    /// ```
    /// use pddl::{Domain, Parser, Problem};
    /// use pddl_validate::Validator;
    ///
    /// let domain = Domain::from_str("(define (domain d) (:predicates (p) (q))
    ///     (:action a :parameters () :precondition (p) :effect (q)))").unwrap();
    /// let problem = Problem::from_str("(define (problem x) (:domain d) (:init) (:goal (q)))").unwrap();
    /// let report = Validator::new(&domain, &problem).unwrap().validate_text("(a)");
    /// let advice = report.violation().unwrap().advice();
    /// assert_eq!(advice, "(a) has an unsatisfied precondition at step 1: (Set (p) to true)");
    /// ```
    #[must_use]
    pub fn advice(&self) -> String {
        let at = |step: &StepRef| format!("{} at step {}", step.action, step.number);
        match (&self.kind, &self.step) {
            (ViolationKind::PreconditionUnsatisfied(unmet), Some(step)) => format!(
                "{} has an unsatisfied precondition at step {}: {}",
                step.action,
                step.number,
                unmet.advice()
            ),
            (ViolationKind::StartConditionUnsatisfied(unmet), Some(step)) => format!(
                "{} has an unsatisfied condition at start at step {}: {}",
                step.action,
                step.number,
                unmet.advice()
            ),
            (ViolationKind::EndConditionUnsatisfied(unmet), Some(step)) => format!(
                "{} has an unsatisfied condition at end at step {}: {}",
                step.action,
                step.number,
                unmet.advice()
            ),
            (ViolationKind::InvariantViolated { time, unmet }, Some(step)) => format!(
                "{} has an unsatisfied invariant at time {time} (step {}): {}",
                step.action,
                step.number,
                unmet.advice()
            ),
            (ViolationKind::DurationConstraint(unmet), Some(step)) => {
                format!("Choose a duration for {} that satisfies {unmet}", at(step))
            }
            (ViolationKind::SelfOverlap { other }, Some(step)) => {
                format!("Start {} after step {other} ends", at(step))
            }
            (ViolationKind::Interference { other, .. }, Some(step)) => format!(
                "Move {} at least epsilon away from {other}, or reorder them",
                at(step)
            ),
            (ViolationKind::GoalUnsatisfied(unmet), _) => {
                format!("The goal is not satisfied: {}", unmet.advice())
            }
            (ViolationKind::UnknownAction(_), Some(step)) => {
                format!("Replace {}: the domain has no such action", at(step))
            }
            (ViolationKind::Arity { expected, .. }, Some(step)) => {
                format!("Give {} exactly {expected} arguments", at(step))
            }
            (ViolationKind::UnknownObject { object, .. }, Some(step)) => {
                format!("Declare `{object}` or replace it in {}", at(step))
            }
            (
                ViolationKind::TypeMismatch {
                    parameter,
                    expected,
                    ..
                },
                Some(step),
            ) => format!(
                "Pass an object of type `{expected}` for ?{parameter} in {}",
                at(step)
            ),
            (ViolationKind::PlanSyntax(_), _) => "Fix the plan file".to_string(),
            (ViolationKind::Undefined(fluent), _) => {
                format!("(Give {fluent} a value in :init or an earlier step)")
            }
            (kind, Some(step)) => format!("{kind} in {}", at(step)),
            (kind, None) => kind.to_string(),
        }
    }
}

impl fmt::Display for Violation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(step) = &self.step {
            write!(f, "step {} {}: ", step.number, step.action)?;
        }

        write!(f, "{}", self.kind)
    }
}

/// A step of the plan, as diagnostics name it.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct StepRef {
    /// The 1-based position of the step in the plan, as VAL counts.
    pub number: usize,
    /// The 1-based line of the plan file the step starts on; 0 for a plan built in code.
    pub line: usize,
    /// The step as written, in lowercase, such as `(drive t1 home shop)`.
    pub action: String,
    /// The start time of the step in a temporal plan; `None` in a sequential one.
    pub time: Option<f64>,
}

/// The kinds of [`Violation`]. All names are in ASCII lowercase.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ViolationKind {
    /// The plan text cannot be read.
    PlanSyntax(crate::PlanError),
    /// The step names an action the domain does not define.
    UnknownAction(String),
    /// The step passes the wrong number of arguments.
    Arity {
        /// The number of parameters of the action.
        expected: usize,
        /// The number of arguments of the step.
        found: usize,
    },
    /// The step passes an object that neither the domain nor the problem declares.
    UnknownObject {
        /// The parameter it is passed for, without `?`.
        parameter: String,
        /// The undeclared object.
        object: String,
    },
    /// The step passes an object of the wrong type.
    TypeMismatch {
        /// The parameter it is passed for, without `?`.
        parameter: String,
        /// The object passed.
        object: String,
        /// The declared type of the parameter, such as `truck` or `(either car truck)`.
        expected: String,
    },
    /// The precondition does not hold in the state before the step.
    PreconditionUnsatisfied(Unmet),
    /// The final state does not satisfy the goal.
    GoalUnsatisfied(Unmet),
    /// An effect, or a condition whose truth value depends on it, reads a numeric fluent
    /// that has no value.
    Undefined(Atom),
    /// The rendered expression divides by zero.
    DivisionByZero(String),
    /// The rendered expression has no finite value.
    NonFinite(String),
    /// The step updates the fluent with `assign`, `scale-up` or `scale-down` together
    /// with another update, so the result would depend on their order.
    ConflictingUpdates(Atom),
    /// A timestamped plan has a step without a timestamp.
    MissingTimestamp,
    /// A durative action has no `[duration]`.
    MissingDuration,
    /// An instantaneous action has a `[duration]`.
    UnexpectedDuration,
    /// The duration is not positive (at most 1e-9), not finite, or rounds away when added
    /// to a huge start time.
    NonPositiveDuration(f64),
    /// A start or end time that is negative or not finite.
    InvalidTime(f64),
    /// The duration violates the action's `:duration` constraint; `left` is the duration.
    DurationConstraint(Unmet),
    /// The step runs while an earlier step of the same action with the same arguments
    /// still runs. Following the formal semantics of PDDL 2.1 the validator is built on
    /// (Abdulaziz and Koller, 2022), a ground durative action must not overlap itself:
    /// its end could belong to either start.
    SelfOverlap {
        /// The 1-based number of the other step, as [`StepRef::number`].
        other: usize,
    },
    /// An `at start` condition does not hold when the action starts.
    StartConditionUnsatisfied(Unmet),
    /// An `at end` condition does not hold when the action ends.
    EndConditionUnsatisfied(Unmet),
    /// An `over all` condition fails at `time`, strictly between the action's start and
    /// end.
    InvariantViolated {
        /// A time at which the condition fails.
        time: f64,
        /// What fails then.
        unmet: Unmet,
    },
    /// The event interferes with another one at the same time or less than the
    /// separation `epsilon` apart (PDDL 2.1 mutex): one changes what the other reads or
    /// changes.
    Interference {
        /// What this step does then: `start`, `end` or `apply`.
        event: String,
        /// The other event, such as `end of (drive t1 a b) (step 2)` or
        /// `timed literal (open shop)`.
        other: String,
        /// The time of this event.
        time: f64,
        /// The time between the two events, smaller than `epsilon`.
        separation: f64,
    },
    /// PDDL+ events kept triggering each other at `time`: after `limit` rounds of firing,
    /// one was still enabled, typically an event whose effect leaves its own precondition
    /// true.
    EventCascade {
        /// When the cascade happened.
        time: f64,
        /// The number of rounds fired.
        limit: usize,
    },
}

impl fmt::Display for ViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlanSyntax(error) => write!(f, "cannot read the plan: {error}"),
            Self::UnknownAction(name) => write!(f, "unknown action `{name}`"),
            Self::Arity { expected, found } => {
                write!(f, "expected {expected} arguments, found {found}")
            }
            Self::UnknownObject { parameter, object } => {
                write!(f, "unknown object `{object}` for ?{parameter}")
            }
            Self::TypeMismatch {
                parameter,
                object,
                expected,
            } => write!(
                f,
                "`{object}` is not of type `{expected}`, as ?{parameter} requires"
            ),
            Self::PreconditionUnsatisfied(unmet) => {
                write!(f, "unsatisfied precondition: {unmet}")
            }
            Self::GoalUnsatisfied(unmet) => write!(f, "goal not satisfied: {unmet}"),
            Self::Undefined(fluent) => write!(f, "{fluent} is undefined"),
            Self::DivisionByZero(exp) => write!(f, "{exp} divides by zero"),
            Self::NonFinite(exp) => write!(f, "{exp} has no finite value"),
            Self::ConflictingUpdates(fluent) => {
                write!(f, "{fluent} is updated twice in incompatible ways")
            }
            Self::MissingTimestamp => f.write_str("the plan is timed but this step has no time"),
            Self::MissingDuration => f.write_str("a durative action needs a [duration]"),
            Self::UnexpectedDuration => {
                f.write_str("an instantaneous action cannot have a [duration]")
            }
            Self::NonPositiveDuration(d) => {
                write!(f, "the duration {d} is not a positive, finite number")
            }
            Self::InvalidTime(t) => write!(f, "the time {t} is not a non-negative, finite number"),
            Self::DurationConstraint(unmet) => {
                write!(f, "the duration violates {unmet}")
            }
            Self::SelfOverlap { other } => {
                write!(
                    f,
                    "overlaps step {other}, the same action with the same arguments"
                )
            }
            Self::StartConditionUnsatisfied(unmet) => {
                write!(f, "unsatisfied condition at start: {unmet}")
            }
            Self::EndConditionUnsatisfied(unmet) => {
                write!(f, "unsatisfied condition at end: {unmet}")
            }
            Self::InvariantViolated { time, unmet } => {
                write!(f, "invariant violated at time {time}: {unmet}")
            }
            Self::EventCascade { time, limit } => {
                write!(
                    f,
                    "events still trigger at time {time} after {limit} rounds"
                )
            }
            Self::Interference {
                event,
                other,
                time,
                separation,
            } => write!(
                f,
                "{event} at time {time} interferes with {other} {separation} apart"
            ),
        }
    }
}

/// The part of a condition that does not hold, as specific as the condition allows.
///
/// A conjunction reports its first failing conjunct, a universal quantifier its first
/// failing instance. A disjunction or existential quantifier has no single culprit and
/// reports [`Unmet::NoneOf`] with each alternative.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Unmet {
    /// A ground atom has the wrong truth value.
    Atom {
        /// The atom.
        atom: Atom,
        /// The truth value the condition needs.
        expected: bool,
        /// `true` if the atom is a derived predicate, so its rule decides its value.
        derived: bool,
    },
    /// An equality `(= a b)` has the wrong truth value.
    Equality {
        /// The left object.
        left: String,
        /// The right object.
        right: String,
        /// `true` if the condition needs the objects to be the same.
        expected: bool,
    },
    /// A numeric comparison has the wrong truth value.
    Comparison {
        /// The comparison, ground, such as `(>= (fuel t1) 5)`.
        condition: String,
        /// The value of the left side.
        left: f64,
        /// The value of the right side.
        right: f64,
        /// The truth value the condition needs.
        expected: bool,
    },
    /// No alternative of a disjunction, existential quantifier or implication holds.
    NoneOf {
        /// The condition, ground as far as the enclosing bindings go.
        condition: String,
        /// Why each alternative fails, at most [`MAX_ALTERNATIVES`] of them.
        alternatives: Vec<Unmet>,
        /// The number of failing alternatives left out of `alternatives`.
        omitted: usize,
    },
}

/// The most alternatives an [`Unmet::NoneOf`] lists. An existential quantifier over many
/// objects would otherwise list every one of them.
pub const MAX_ALTERNATIVES: usize = 8;

impl Unmet {
    /// Returns a repair hint, such as `(Set (at t1 shop) to true)`.
    #[must_use]
    pub fn advice(&self) -> String {
        match self {
            Self::Atom {
                atom,
                expected,
                derived: false,
            } => format!("(Set {atom} to {expected})"),
            Self::Atom {
                atom,
                expected,
                derived: true,
            } => format!("(Make the rule of derived {atom} yield {expected})"),
            Self::Equality {
                left,
                right,
                expected: true,
            } => format!("(Use the same object for {left} and {right})"),
            Self::Equality {
                left,
                expected: false,
                ..
            } => format!("(Use different objects instead of {left} twice)"),
            Self::Comparison {
                condition,
                left,
                right,
                expected,
            } => format!("(Make {condition} {expected}: the sides are {left} and {right})"),
            Self::NoneOf { alternatives, .. } if alternatives.is_empty() => {
                String::from("(No binding exists; declare a matching object)")
            }
            Self::NoneOf { alternatives, .. } => {
                let hints: Vec<String> = alternatives.iter().map(Self::advice).collect();
                format!("one of: {}", hints.join(" or "))
            }
        }
    }
}

impl fmt::Display for Unmet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Atom {
                atom,
                expected: true,
                ..
            } => write!(f, "{atom}"),
            Self::Atom { atom, .. } => write!(f, "(not {atom})"),
            Self::Equality {
                left,
                right,
                expected: true,
            } => write!(f, "(= {left} {right})"),
            Self::Equality { left, right, .. } => write!(f, "(not (= {left} {right}))"),
            Self::Comparison {
                condition,
                expected: true,
                ..
            }
            | Self::NoneOf { condition, .. } => write!(f, "{condition}"),
            Self::Comparison { condition, .. } => write!(f, "(not {condition})"),
        }
    }
}

/// The ground changes one step made to the state.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct StepTrace {
    /// The step, in a sequential plan.
    pub step: Option<StepRef>,
    /// The time of the happening, in a temporal plan.
    pub time: Option<f64>,
    /// The events of the happening, in a temporal plan, such as `start (drive t1 a b)`,
    /// in plan order.
    pub events: Vec<String>,
    /// Atoms that were false and are true after the step, sorted.
    pub added: Vec<Atom>,
    /// Atoms that were true and are false after the step, sorted.
    pub deleted: Vec<Atom>,
    /// The numeric fluents the step updated, sorted.
    pub numeric: Vec<FluentChange>,
}

/// A numeric fluent a step updated.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub struct FluentChange {
    /// The fluent, such as `(total-cost)`.
    pub fluent: Atom,
    /// The value before the step; `None` if it was undefined.
    pub before: Option<f64>,
    /// The value after the step.
    pub after: f64,
}

/// The value of the problem's `:metric` for a valid plan.
#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub struct Metric {
    /// Whether the problem minimizes or maximizes the metric.
    pub optimization: pddl::Optimization,
    /// The value in the final state.
    pub value: f64,
}

/// A construct the validator does not interpret, and where it is.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct Unsupported {
    /// The construct.
    pub construct: Construct,
    /// Where it is, such as ``action `drive` `` or `plan`.
    pub location: String,
}

impl Unsupported {
    /// Builds a finding at `location`.
    pub(crate) fn new(construct: Construct, location: impl Into<String>) -> Self {
        Self {
            construct,
            location: location.into(),
        }
    }
}

impl fmt::Display for Unsupported {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} are not supported", self.location, self.construct)
    }
}

/// The PDDL constructs the validator does not interpret yet.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Construct {
    /// A requirement flag the `pddl` parser does not know.
    UnknownRequirement(String),
    /// PDDL 1.2 `:extends`.
    DomainExtension,
    /// PDDL 1.2 `:timeless` facts.
    Timeless,
    /// Continuous change whose rate depends on a continuously changing fluent, or an
    /// `over all` condition that is not linear in time (level 4 dynamics).
    NonlinearDynamics,
    /// Derived predicates in a temporal plan, whose interference this validator does not
    /// decide.
    DerivedPredicatesInTemporalPlans,
    /// A conditional effect of a durative action with an `over all` condition, or with an
    /// `at end` condition on an `at start` effect.
    TimedEffectConditions,
    /// An assignment nested inside a numeric expression of a durative action.
    NestedAssignments,
    /// A PDDL+ process effect that is not continuous, such as `(at start ...)` or `when`.
    DiscreteProcessEffects,
    /// PDDL+ dynamics that switch processes or fire events without end, such as a
    /// process that turns itself off and on again at one instant (Zeno behaviour).
    /// Also reported when legal dynamics need more than the validator's segment limit
    /// (100 000 process switches or event firings between two plan happenings), so a
    /// very busy model is unsupported rather than validated.
    ZenoDynamics,
    /// State-trajectory constraints.
    Constraints,
    /// Preferences, and `is-violated` in a metric.
    Preferences,
    /// Object-valued fluents.
    ObjectFluents,
    /// Derived predicates that depend on their own negation, so no stratification exists.
    UnstratifiedDerivedPredicates,
    /// Effects on `=`. The static check rejects them as malformed, so validation never
    /// meets one; should it, the plan is not accepted.
    EqualityEffects,
}

impl fmt::Display for Construct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownRequirement(name) => write!(f, "requirements like `{name}`"),
            Self::DomainExtension => f.write_str("domain extensions"),
            Self::Timeless => f.write_str("timeless facts"),
            Self::NonlinearDynamics => f.write_str("nonlinear continuous dynamics"),
            Self::DerivedPredicatesInTemporalPlans => {
                f.write_str("derived predicates in temporal plans")
            }
            Self::TimedEffectConditions => {
                f.write_str("these timed conditions of conditional effects")
            }
            Self::NestedAssignments => f.write_str("nested assignments"),
            Self::DiscreteProcessEffects => f.write_str("discrete effects of processes"),
            Self::ZenoDynamics => f.write_str("dynamics that switch without end"),
            Self::Constraints => f.write_str("constraints"),
            Self::Preferences => f.write_str("preferences"),
            Self::ObjectFluents => f.write_str("object fluents"),
            Self::UnstratifiedDerivedPredicates => {
                f.write_str("derived predicates that depend on their own negation")
            }
            Self::EqualityEffects => f.write_str("effects on `=`"),
        }
    }
}

#[cfg(test)]
mod tests {
    //! Pins the exact text of every diagnostic, as users read it.

    use alloc::string::ToString;
    use alloc::vec;

    use super::*;
    use crate::Plan;

    /// Returns step 3, `(drive t1 a b)`, of a sequential plan.
    fn step() -> StepRef {
        StepRef {
            number: 3,
            line: 0,
            action: "(drive t1 a b)".to_string(),
            time: None,
        }
    }

    /// Returns the unmet atom `(at t1 a)`, which needs to be true.
    fn at() -> Unmet {
        Unmet::Atom {
            atom: Atom::new("at", ["t1", "a"]),
            expected: true,
            derived: false,
        }
    }

    /// Returns the syntax error of the plan text `(a`.
    #[expect(
        clippy::expect_used,
        reason = "a test fails loudly on a broken fixture"
    )]
    fn syntax_error() -> crate::PlanError {
        "(a".parse::<Plan>().expect_err("`(a` is not closed")
    }

    /// Returns the advice for `kind`, tied to [`step`] when `with_step` holds.
    fn advice(kind: ViolationKind, with_step: bool) -> String {
        let step = with_step.then(step);
        Violation { step, kind }.advice()
    }

    #[test]
    fn report_accessors_match_the_outcome() {
        let unsupported = Unsupported::new(Construct::Constraints, "problem");
        let report = Report::new(Outcome::Unsupported(unsupported.clone()), vec![]);
        assert_eq!(report.status(), ValidationStatus::Unsupported);
        assert_eq!(report.unsupported(), Some(&unsupported));
        assert_eq!(report.violation(), None);
        assert_eq!(report.outcome(), &Outcome::Unsupported(unsupported));

        let report = Report::new(Outcome::Valid, vec![]);
        assert_eq!(report.unsupported(), None);
        assert_eq!(report.violation(), None);
    }

    #[test]
    fn violation_kinds_display_their_diagnostic() {
        let atom = Atom::new("fuel", ["t1"]);
        let cases = [
            (
                ViolationKind::PlanSyntax(syntax_error()),
                "cannot read the plan: line 1, column 1: `(` is never closed",
            ),
            (
                ViolationKind::UnknownAction("fly".to_string()),
                "unknown action `fly`",
            ),
            (
                ViolationKind::Arity {
                    expected: 2,
                    found: 1,
                },
                "expected 2 arguments, found 1",
            ),
            (
                ViolationKind::UnknownObject {
                    parameter: "t".to_string(),
                    object: "t9".to_string(),
                },
                "unknown object `t9` for ?t",
            ),
            (
                ViolationKind::TypeMismatch {
                    parameter: "t".to_string(),
                    object: "home".to_string(),
                    expected: "truck".to_string(),
                },
                "`home` is not of type `truck`, as ?t requires",
            ),
            (
                ViolationKind::Undefined(atom.clone()),
                "(fuel t1) is undefined",
            ),
            (
                ViolationKind::DivisionByZero("(/ 1 (fuel t1))".to_string()),
                "(/ 1 (fuel t1)) divides by zero",
            ),
            (
                ViolationKind::NonFinite("(* 1e308 10)".to_string()),
                "(* 1e308 10) has no finite value",
            ),
            (
                ViolationKind::ConflictingUpdates(atom),
                "(fuel t1) is updated twice in incompatible ways",
            ),
            (
                ViolationKind::MissingTimestamp,
                "the plan is timed but this step has no time",
            ),
            (
                ViolationKind::MissingDuration,
                "a durative action needs a [duration]",
            ),
            (
                ViolationKind::UnexpectedDuration,
                "an instantaneous action cannot have a [duration]",
            ),
            (
                ViolationKind::NonPositiveDuration(-1.5),
                "the duration -1.5 is not a positive, finite number",
            ),
            (
                ViolationKind::InvalidTime(-2.0),
                "the time -2 is not a non-negative, finite number",
            ),
            (
                ViolationKind::SelfOverlap { other: 1 },
                "overlaps step 1, the same action with the same arguments",
            ),
            (
                ViolationKind::Interference {
                    event: "start".to_string(),
                    other: "end of (load t1) (step 2)".to_string(),
                    time: 4.0,
                    separation: 0.0,
                },
                "start at time 4 interferes with end of (load t1) (step 2) 0 apart",
            ),
            (
                ViolationKind::EventCascade {
                    time: 2.5,
                    limit: 1000,
                },
                "events still trigger at time 2.5 after 1000 rounds",
            ),
        ];

        for (kind, text) in cases {
            assert_eq!(kind.to_string(), text);
        }
    }

    #[test]
    fn condition_violations_display_the_unmet_part() {
        let cases = [
            (
                ViolationKind::PreconditionUnsatisfied(at()),
                "unsatisfied precondition: (at t1 a)",
            ),
            (
                ViolationKind::GoalUnsatisfied(at()),
                "goal not satisfied: (at t1 a)",
            ),
            (
                ViolationKind::DurationConstraint(at()),
                "the duration violates (at t1 a)",
            ),
            (
                ViolationKind::StartConditionUnsatisfied(at()),
                "unsatisfied condition at start: (at t1 a)",
            ),
            (
                ViolationKind::EndConditionUnsatisfied(at()),
                "unsatisfied condition at end: (at t1 a)",
            ),
            (
                ViolationKind::InvariantViolated {
                    time: 1.5,
                    unmet: at(),
                },
                "invariant violated at time 1.5: (at t1 a)",
            ),
        ];

        for (kind, text) in cases {
            assert_eq!(kind.to_string(), text);
        }
    }

    #[test]
    fn violation_display_names_the_step() {
        let violation = Violation {
            step: Some(step()),
            kind: ViolationKind::MissingDuration,
        };
        assert_eq!(
            violation.to_string(),
            "step 3 (drive t1 a b): a durative action needs a [duration]"
        );
    }

    #[test]
    fn step_violations_advise_a_repair() {
        let cases = [
            (
                ViolationKind::StartConditionUnsatisfied(at()),
                "(drive t1 a b) has an unsatisfied condition at start at step 3: \
                 (Set (at t1 a) to true)",
            ),
            (
                ViolationKind::EndConditionUnsatisfied(at()),
                "(drive t1 a b) has an unsatisfied condition at end at step 3: \
                 (Set (at t1 a) to true)",
            ),
            (
                ViolationKind::InvariantViolated {
                    time: 1.5,
                    unmet: at(),
                },
                "(drive t1 a b) has an unsatisfied invariant at time 1.5 (step 3): \
                 (Set (at t1 a) to true)",
            ),
            (
                ViolationKind::DurationConstraint(at()),
                "Choose a duration for (drive t1 a b) at step 3 that satisfies (at t1 a)",
            ),
            (
                ViolationKind::SelfOverlap { other: 1 },
                "Start (drive t1 a b) at step 3 after step 1 ends",
            ),
            (
                ViolationKind::Interference {
                    event: "start".to_string(),
                    other: "timed literal (open shop)".to_string(),
                    time: 4.0,
                    separation: 0.0,
                },
                "Move (drive t1 a b) at step 3 at least epsilon away from \
                 timed literal (open shop), or reorder them",
            ),
            (
                ViolationKind::UnknownAction("fly".to_string()),
                "Replace (drive t1 a b) at step 3: the domain has no such action",
            ),
            (
                ViolationKind::Arity {
                    expected: 2,
                    found: 1,
                },
                "Give (drive t1 a b) at step 3 exactly 2 arguments",
            ),
            (
                ViolationKind::UnknownObject {
                    parameter: "t".to_string(),
                    object: "t9".to_string(),
                },
                "Declare `t9` or replace it in (drive t1 a b) at step 3",
            ),
            (
                ViolationKind::TypeMismatch {
                    parameter: "t".to_string(),
                    object: "home".to_string(),
                    expected: "truck".to_string(),
                },
                "Pass an object of type `truck` for ?t in (drive t1 a b) at step 3",
            ),
            (
                ViolationKind::Undefined(Atom::new("fuel", ["t1"])),
                "(Give (fuel t1) a value in :init or an earlier step)",
            ),
            (
                ViolationKind::MissingDuration,
                "a durative action needs a [duration] in (drive t1 a b) at step 3",
            ),
        ];

        for (kind, text) in cases {
            assert_eq!(advice(kind, true), text);
        }
    }

    #[test]
    fn plan_wide_violations_advise_a_repair() {
        assert_eq!(
            advice(ViolationKind::PlanSyntax(syntax_error()), false),
            "Fix the plan file"
        );
        assert_eq!(
            advice(ViolationKind::GoalUnsatisfied(at()), false),
            "The goal is not satisfied: (Set (at t1 a) to true)"
        );
        assert_eq!(
            advice(
                ViolationKind::EventCascade {
                    time: 2.5,
                    limit: 1000
                },
                false
            ),
            "events still trigger at time 2.5 after 1000 rounds"
        );
    }

    #[test]
    fn unmet_conditions_display_and_advise() {
        let derived = Unmet::Atom {
            atom: Atom::new("reachable", ["a"]),
            expected: false,
            derived: true,
        };
        let same = Unmet::Equality {
            left: "a".to_string(),
            right: "b".to_string(),
            expected: true,
        };
        let different = Unmet::Equality {
            left: "a".to_string(),
            right: "a".to_string(),
            expected: false,
        };
        let comparison = Unmet::Comparison {
            condition: "(>= (fuel t1) 5)".to_string(),
            left: 2.0,
            right: 5.0,
            expected: true,
        };
        let negated = Unmet::Comparison {
            condition: "(>= (fuel t1) 5)".to_string(),
            left: 7.0,
            right: 5.0,
            expected: false,
        };
        let none = Unmet::NoneOf {
            condition: "(exists (?t - truck) (at ?t a))".to_string(),
            alternatives: vec![],
            omitted: 0,
        };
        let either = Unmet::NoneOf {
            condition: "(or (at t1 a) (reachable a))".to_string(),
            alternatives: vec![at(), same.clone()],
            omitted: 0,
        };

        let cases = [
            (
                derived,
                "(not (reachable a))",
                "(Make the rule of derived (reachable a) yield false)",
            ),
            (same, "(= a b)", "(Use the same object for a and b)"),
            (
                different,
                "(not (= a a))",
                "(Use different objects instead of a twice)",
            ),
            (
                comparison,
                "(>= (fuel t1) 5)",
                "(Make (>= (fuel t1) 5) true: the sides are 2 and 5)",
            ),
            (
                negated,
                "(not (>= (fuel t1) 5))",
                "(Make (>= (fuel t1) 5) false: the sides are 7 and 5)",
            ),
            (
                none,
                "(exists (?t - truck) (at ?t a))",
                "(No binding exists; declare a matching object)",
            ),
            (
                either,
                "(or (at t1 a) (reachable a))",
                "one of: (Set (at t1 a) to true) or (Use the same object for a and b)",
            ),
        ];

        for (unmet, text, hint) in cases {
            assert_eq!(
                (unmet.to_string(), unmet.advice()),
                (text.to_string(), hint.to_string())
            );
        }
    }

    #[test]
    fn constructs_display_their_name() {
        let cases = [
            (
                Construct::UnknownRequirement(":fancy".to_string()),
                "requirements like `:fancy`",
            ),
            (Construct::DomainExtension, "domain extensions"),
            (Construct::Timeless, "timeless facts"),
            (
                Construct::NonlinearDynamics,
                "nonlinear continuous dynamics",
            ),
            (
                Construct::DerivedPredicatesInTemporalPlans,
                "derived predicates in temporal plans",
            ),
            (
                Construct::TimedEffectConditions,
                "these timed conditions of conditional effects",
            ),
            (Construct::NestedAssignments, "nested assignments"),
            (
                Construct::DiscreteProcessEffects,
                "discrete effects of processes",
            ),
            (Construct::ZenoDynamics, "dynamics that switch without end"),
            (Construct::Constraints, "constraints"),
            (Construct::Preferences, "preferences"),
            (Construct::ObjectFluents, "object fluents"),
            (
                Construct::UnstratifiedDerivedPredicates,
                "derived predicates that depend on their own negation",
            ),
            (Construct::EqualityEffects, "effects on `=`"),
        ];

        for (construct, text) in cases {
            assert_eq!(construct.to_string(), text);
        }

        assert_eq!(
            Unsupported::new(Construct::Timeless, "domain").to_string(),
            "domain: timeless facts are not supported"
        );
    }
}
