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
}

impl Report {
    /// Builds a report.
    pub(crate) const fn new(outcome: Outcome, trace: Vec<StepTrace>) -> Self {
        Self { outcome, trace }
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
    /// A valid plan has one entry per step. An invalid one has an entry for each step
    /// before the failing one. An unsupported one has none.
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct StepRef {
    /// The 1-based position of the step in the plan, as VAL counts.
    pub number: usize,
    /// The 1-based line of the plan file the step starts on; 0 for a plan built in code.
    pub line: usize,
    /// The step as written, in lowercase, such as `(drive t1 home shop)`.
    pub action: String,
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
            Self::NoneOf { condition, .. } => write!(f, "{condition}"),
        }
    }
}

/// The ground changes one step made to the state.
#[derive(Debug, Clone, PartialEq, Default)]
#[non_exhaustive]
pub struct StepTrace {
    /// The step.
    pub step: Option<StepRef>,
    /// Atoms that were false and are true after the step, sorted.
    pub added: Vec<Atom>,
    /// Atoms that were true and are false after the step, sorted.
    pub deleted: Vec<Atom>,
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
    /// Durative actions in the domain (level 3).
    DurativeActions,
    /// Timestamps or durations in the plan (level 3).
    TemporalPlans,
    /// Timed initial literals (level 3).
    TimedInitialLiterals,
    /// State-trajectory constraints.
    Constraints,
    /// Preferences, and `is-violated` in a metric.
    Preferences,
    /// Object-valued fluents.
    ObjectFluents,
    /// Numeric fluents and action costs (level 2).
    NumericFluents,
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
            Self::DurativeActions => f.write_str("durative actions"),
            Self::TemporalPlans => f.write_str("temporal plans"),
            Self::TimedInitialLiterals => f.write_str("timed initial literals"),
            Self::Constraints => f.write_str("constraints"),
            Self::Preferences => f.write_str("preferences"),
            Self::ObjectFluents => f.write_str("object fluents"),
            Self::NumericFluents => f.write_str("numeric fluents"),
            Self::UnstratifiedDerivedPredicates => {
                f.write_str("derived predicates that depend on their own negation")
            }
            Self::EqualityEffects => f.write_str("effects on `=`"),
        }
    }
}
