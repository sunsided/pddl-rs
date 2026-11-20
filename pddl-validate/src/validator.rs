//! The validator: checks a domain and problem once, then runs plans against them.
//!
//! ```mermaid
//! flowchart TD
//!     new["Validator::new"] --> names["check names, arities, types"]
//!     names --> scan["find unsupported constructs"]
//!     scan --> strata["stratify derived predicates"]
//!     strata --> init["build the initial state"]
//!     init --> step{"next step?"}
//!     step -- yes --> ground["ground: action, arity, objects, types"]
//!     ground --> pre["check the precondition"]
//!     pre --> eff["collect effects, delete, add"]
//!     eff --> close["recompute derived atoms"]
//!     close --> step
//!     step -- no --> goal["check the goal"]
//! ```

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use pddl::{AtomicFormula, Domain, InitElement, Literal, Problem};

use crate::atom::{lower, Atom};
use crate::derived;
use crate::effects::Changes;
use crate::error::ModelError;
use crate::eval::{EvalError, Evaluator, Scope};
use crate::model::check::{self, Issue};
use crate::model::universe::type_text;
use crate::model::{self as model, Model};
use crate::plan::{Plan, PlanErrorKind, PlanStep};
use crate::report::{
    Construct, Metric, Outcome, Report, StepRef, StepTrace, Unsupported, Violation, ViolationKind,
};
use crate::state::State;

/// Validates plans for one domain and problem.
///
/// Construction checks the domain and problem once; [`Validator::validate`] can then run
/// any number of plans. Validation never panics, whatever the plan.
///
/// # Examples
///
/// ```
/// use pddl::{Domain, Parser, Problem};
/// use pddl_validate::{ValidationStatus, Validator};
///
/// let domain = Domain::from_str(
///     "(define (domain switch) (:predicates (on))
///        (:action flip :parameters () :precondition (not (on)) :effect (on)))",
/// ).unwrap();
/// let problem = Problem::from_str(
///     "(define (problem light) (:domain switch) (:init) (:goal (on)))",
/// ).unwrap();
/// let validator = Validator::new(&domain, &problem).unwrap();
///
/// assert_eq!(validator.validate_text("(flip)").status(), ValidationStatus::Valid);
///
/// let report = validator.validate_text("(flip)\n(flip)");
/// assert_eq!(report.status(), ValidationStatus::Invalid);
/// assert_eq!(report.violation().unwrap().to_string(),
///     "step 2 (flip): unsatisfied precondition: (not (on))");
/// ```
#[derive(Debug)]
pub struct Validator<'a> {
    /// The checked task, or why the validator cannot interpret it.
    inner: Inner<'a>,
    /// The relative tolerance of numeric comparisons; see [`Validator::with_tolerance`].
    tolerance: f64,
}

/// The default relative tolerance of numeric comparisons: wide enough to absorb the
/// rounding of a few thousand `f64` additions, far below any meaningful difference.
pub const DEFAULT_TOLERANCE: f64 = 1e-9;

/// What [`Validator::new`] found.
#[derive(Debug)]
enum Inner<'a> {
    /// A task the validator interprets.
    Ready {
        /// The indexed domain and problem.
        model: Model<'a>,
        /// The problem, for its goal.
        problem: &'a Problem,
        /// The initial state, derived atoms not yet computed.
        initial: State,
    },
    /// A task that uses a construct the validator does not interpret.
    Unsupported(Unsupported),
}

/// Why a step stops the plan.
enum Stop {
    /// The step is invalid.
    Invalid(ViolationKind),
    /// The step meets a construct the validator does not interpret.
    Unsupported(Construct),
}

impl From<EvalError> for Stop {
    fn from(error: EvalError) -> Self {
        match error {
            EvalError::Unsupported(construct) => Self::Unsupported(construct),
            EvalError::Undefined(fluent) => Self::Invalid(ViolationKind::Undefined(fluent)),
            EvalError::DivisionByZero(exp) => Self::Invalid(ViolationKind::DivisionByZero(exp)),
            EvalError::NonFinite(exp) => Self::Invalid(ViolationKind::NonFinite(exp)),
            EvalError::ConflictingUpdates(fluent) => {
                Self::Invalid(ViolationKind::ConflictingUpdates(fluent))
            }
        }
    }
}

impl<'a> Validator<'a> {
    /// Checks `domain` and `problem` and prepares the initial state.
    ///
    /// A task that uses a construct the validator does not interpret yet, such as durative
    /// actions, still builds a validator, whose every report is
    /// [`ValidationStatus::Unsupported`](crate::ValidationStatus::Unsupported).
    ///
    /// # Errors
    ///
    /// A [`ModelError`] if the problem is for another domain, or if a formula uses an
    /// undeclared type, predicate, function, object or variable, applies a symbol to the
    /// wrong number of arguments, or sets a derived predicate or `=`; also for two actions
    /// of one name.
    pub fn new(domain: &'a Domain, problem: &'a Problem) -> Result<Self, ModelError> {
        model::check_domain_name(domain, problem)?;
        if let Some(unsupported) = model::unsupported_sections(domain, problem) {
            return Ok(Self::unsupported(unsupported));
        }

        let mut model = Model::new(domain, problem)?;
        let checked =
            check::check_function_types(domain).and_then(|()| check::check(&model, problem));
        match checked {
            Err(Issue::Model(error)) => return Err(error),
            Err(Issue::Unsupported(unsupported)) => return Ok(Self::unsupported(unsupported)),
            Ok(()) => {}
        }

        if let Err(unsupported) = derived::stratify(&mut model) {
            return Ok(Self::unsupported(unsupported));
        }

        let initial = initial_state(&model, problem);

        Ok(Self {
            inner: Inner::Ready {
                model,
                problem,
                initial,
            },
            tolerance: DEFAULT_TOLERANCE,
        })
    }

    /// Builds a validator that reports `unsupported` for every plan.
    const fn unsupported(unsupported: Unsupported) -> Self {
        Self {
            inner: Inner::Unsupported(unsupported),
            tolerance: DEFAULT_TOLERANCE,
        }
    }

    /// Sets the relative tolerance of numeric comparisons, [`DEFAULT_TOLERANCE`] unless
    /// set.
    ///
    /// Two values `a` and `b` are equal if `|a - b| <= tolerance * max(1, |a|, |b|)`;
    /// otherwise they compare as numbers. So `(= (x) 0.3)` holds after `(x)` was set to
    /// 0.1 and increased by 0.2, and exactly one of `<`, `=` and `>` holds for any pair.
    /// A tolerance of 0 compares exactly; a negative or NaN tolerance counts as 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use pddl::{Domain, Parser, Problem};
    /// use pddl_validate::{ValidationStatus, Validator};
    ///
    /// let domain = Domain::from_str("(define (domain d) (:predicates (done)) (:functions (x))
    ///     (:action add :parameters () :effect (increase (x) 0.2))
    ///     (:action check :parameters () :precondition (= (x) 0.3) :effect (done)))").unwrap();
    /// let problem = Problem::from_str("(define (problem p) (:domain d)
    ///     (:init (= (x) 0.1)) (:goal (done)))").unwrap();
    /// let validator = Validator::new(&domain, &problem).unwrap();
    /// assert_eq!(validator.validate_text("(add) (check)").status(), ValidationStatus::Valid);
    ///
    /// let exact = validator.with_tolerance(0.0);
    /// assert_eq!(exact.validate_text("(add) (check)").status(), ValidationStatus::Invalid);
    /// ```
    #[must_use]
    pub fn with_tolerance(mut self, tolerance: f64) -> Self {
        self.tolerance = if tolerance > 0.0 { tolerance } else { 0.0 };
        self
    }

    /// Parses `text` as a plan file and validates it.
    ///
    /// A plan file that cannot be read is [`ValidationStatus::Invalid`](crate::ValidationStatus::Invalid)
    /// with [`ViolationKind::PlanSyntax`]; one with timestamps is
    /// [`ValidationStatus::Unsupported`](crate::ValidationStatus::Unsupported), since
    /// temporal plans are not validated yet.
    #[must_use]
    pub fn validate_text(&self, text: &str) -> Report {
        match text.parse::<Plan>() {
            Ok(plan) => self.validate(&plan),
            Err(error) if error.kind == PlanErrorKind::Timestamped => {
                let location = format!("plan line {}", error.line);
                let unsupported = Unsupported::new(Construct::TemporalPlans, location);
                Report::new(Outcome::Unsupported(unsupported), Vec::new())
            }
            Err(error) => {
                let violation = Violation {
                    step: None,
                    kind: ViolationKind::PlanSyntax(error),
                };
                Report::new(Outcome::Invalid(violation), Vec::new())
            }
        }
    }

    /// Runs `plan` from the initial state and checks the goal in the final state.
    ///
    /// Validation stops at the first violation, which the report names together with the
    /// step, the failing condition and a repair hint.
    #[must_use]
    pub fn validate(&self, plan: &Plan) -> Report {
        let (model, problem, initial) = match &self.inner {
            Inner::Ready {
                model,
                problem,
                initial,
            } => (model, problem, initial),
            Inner::Unsupported(unsupported) => {
                return Report::new(Outcome::Unsupported(unsupported.clone()), Vec::new());
            }
        };

        let tolerance = self.tolerance;
        let mut state = initial.clone();
        let mut trace = Vec::new();
        if let Err(error) = derived::close(model, &mut state, tolerance) {
            return stopped(Stop::from(error), None, String::from("init"), trace);
        }

        for (index, step) in plan.steps().iter().enumerate() {
            let at = StepRef {
                number: index + 1,
                line: step.line(),
                action: alloc::string::ToString::to_string(step),
            };
            match run_step(model, &mut state, step, tolerance) {
                Ok(mut changes) => {
                    changes.step = Some(at);
                    trace.push(changes);
                }
                Err(stop) => {
                    let location = format!("step {}", at.number);
                    return stopped(stop, Some(at), location, trace);
                }
            }
        }

        let evaluator = Evaluator {
            model,
            state: &state,
            tolerance,
        };
        match evaluator.preconditions(problem.goals(), &mut Scope::new()) {
            Ok(None) => {}
            Ok(Some(unmet)) => {
                let kind = ViolationKind::GoalUnsatisfied(unmet);
                return stopped(Stop::Invalid(kind), None, String::from("goal"), trace);
            }
            Err(error) => return stopped(Stop::from(error), None, String::from("goal"), trace),
        }

        let Some(spec) = problem.metric_spec() else {
            return Report::new(Outcome::Valid, trace);
        };

        match evaluator.metric(spec.expression(), plan.len()) {
            Ok(value) => Report::new(Outcome::Valid, trace).with_metric(Metric {
                optimization: spec.optimization(),
                value,
            }),
            Err(error) => stopped(Stop::from(error), None, String::from("metric"), trace),
        }
    }
}

/// Builds the report of a plan that stopped at `step` (or at the goal, for `None`).
fn stopped(reason: Stop, step: Option<StepRef>, location: String, trace: Vec<StepTrace>) -> Report {
    match reason {
        Stop::Invalid(kind) => Report::new(Outcome::Invalid(Violation { step, kind }), trace),
        Stop::Unsupported(construct) => {
            let unsupported = Unsupported::new(construct, location);
            Report::new(Outcome::Unsupported(unsupported), Vec::new())
        }
    }
}

/// Applies one step to `state` and returns what it changed.
fn run_step(
    model: &Model<'_>,
    state: &mut State,
    step: &PlanStep,
    tolerance: f64,
) -> Result<StepTrace, Stop> {
    let (action, mut scope) = bind(model, step)?;
    let evaluator = Evaluator {
        model,
        state: &*state,
        tolerance,
    };
    if let Some(unmet) = evaluator.preconditions(action.precondition(), &mut scope)? {
        return Err(Stop::Invalid(ViolationKind::PreconditionUnsatisfied(unmet)));
    }

    let mut changes = Changes::default();
    if let Some(effects) = action.effect() {
        changes.collect(&evaluator, effects, &mut scope)?;
    }

    let trace = changes.apply(state)?;
    derived::close(model, state, tolerance)?;

    Ok(trace)
}

/// Looks up the action of `step` and binds its parameters to the step's objects.
fn bind<'m>(
    model: &'m Model<'_>,
    step: &PlanStep,
) -> Result<(&'m pddl::ActionDefinition, Scope), Stop> {
    let invalid = |kind| Err(Stop::Invalid(kind));
    let Some(action) = model.actions.get(step.action()) else {
        return invalid(ViolationKind::UnknownAction(String::from(step.action())));
    };

    let parameters = action.parameters();
    if parameters.len() != step.arguments().len() {
        return invalid(ViolationKind::Arity {
            expected: parameters.len(),
            found: step.arguments().len(),
        });
    }

    let mut scope = Scope::new();
    for (parameter, object) in parameters.iter().zip(step.arguments()) {
        let name = lower(parameter.value());
        if !model.universe.contains(object) {
            return invalid(ViolationKind::UnknownObject {
                parameter: name,
                object: object.clone(),
            });
        }

        if !model.universe.is_of(object, parameter.type_()) {
            return invalid(ViolationKind::TypeMismatch {
                parameter: name,
                object: object.clone(),
                expected: type_text(parameter.type_()),
            });
        }

        scope.push((name, object.clone()));
    }

    Ok((action, scope))
}

/// Builds the initial state from the problem's `:init`.
///
/// Negative literals are skipped: under the closed-world assumption they hold anyway.
/// A nullary `total-cost` function that `:init` gives no value starts at 0, as Fast
/// Downward assumes. The static check has rejected every element this validator cannot
/// interpret.
fn initial_state(model: &Model<'_>, problem: &Problem) -> State {
    let mut state = State::default();
    if model.functions.get(TOTAL_COST) == Some(&0) {
        let total_cost = Atom::lowered(String::from(TOTAL_COST), Vec::new());
        state.fluents.insert(total_cost, 0.0);
    }

    for element in problem.init().iter() {
        match element {
            InitElement::Literal(Literal::AtomicFormula(AtomicFormula::Predicate(atom))) => {
                let arguments = atom.values().iter().map(|name| lower(name)).collect();
                let atom = Atom::lowered(lower(atom.predicate()), arguments);
                state.facts.insert(atom);
            }
            InitElement::IsValue(term, value) => {
                let arguments = term.names().iter().map(|name| lower(name)).collect();
                let fluent = Atom::lowered(lower(term.symbol()), arguments);
                state.fluents.insert(fluent, **value);
            }
            InitElement::Literal(_) | InitElement::At(..) | InitElement::IsObject(..) => {}
        }
    }

    state
}

/// The function that action costs accumulate in.
const TOTAL_COST: &str = "total-cost";
