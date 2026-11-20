//! Evaluation of conditions in a state, with an explanation when they fail.
//!
//! [`Evaluator::holds`] checks a goal description against a wanted truth value and,
//! when it does not have it, returns the most specific part to blame ([`Unmet`]).
//! Negation flips the wanted value instead of evaluating and inverting, so a failing
//! `(not (or a b))` blames `a` or `b` rather than the whole disjunction.
//!
//! Conditions are three-valued (Kleene logic). A part whose value is not defined, because
//! it reads a fluent without a value, divides by zero or leaves the finite range, is
//! unknown ([`EvalError::is_unknown`]). A conjunction-like condition (`and`, `forall`, a
//! precondition list) fails if a part definitely fails, and is otherwise unknown if a part
//! is unknown. A disjunction-like condition (`or`, `exists`, `imply`) holds if a part
//! definitely holds, and is otherwise unknown if a part is unknown. Negation keeps an
//! unknown unknown. So `(or (p) (> (f) 0))` holds when `(p)` does, whatever `f`. An
//! unknown condition is reported as its first unknown part's [`EvalError`]. Any other
//! [`EvalError`], such as [`EvalError::Unsupported`], is fatal and wins over every
//! verdict.
//!
//! Evaluation never short-circuits: every part of a condition is evaluated, so the
//! verdict does not depend on the order of its parts. [`numeric`] evaluates numeric
//! expressions and comparisons.

pub mod numeric;

use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;

use pddl::{
    AtomicFormula, GoalDescription, Literal, PreconditionGoalDescription,
    PreconditionGoalDescriptions, PredicateAtomicFormula, PreferenceGoalDescription, Term,
    TypedVariables,
};

use crate::atom::{lower, Atom};
use crate::model::Model;
use crate::render;
use crate::report::{Construct, Unmet, MAX_ALTERNATIVES};
use crate::state::State;

/// Variable bindings, innermost last: lowercased variable name (without `?`) and object.
pub type Scope = Vec<(String, String)>;

/// Returns the object bound to `variable` in `scope`, innermost binding first.
pub fn lookup<'s>(scope: &'s Scope, variable: &str) -> Option<&'s str> {
    scope
        .iter()
        .rev()
        .find(|(name, _)| name == variable)
        .map(|(_, object)| object.as_str())
}

/// Why a condition or effect could not be evaluated at all.
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    /// The condition uses a construct the validator does not interpret.
    Unsupported(Construct),
    /// A fluent without a value is read.
    Undefined(Atom),
    /// The rendered expression divides by zero.
    DivisionByZero(String),
    /// The rendered expression has no finite value.
    NonFinite(String),
    /// One step updates a fluent with `assign` or a scaling together with another update.
    ConflictingUpdates(Atom),
}

impl EvalError {
    /// Returns `true` for an error that only says a value is undefined: an
    /// [`Self::Undefined`] fluent, a [`Self::DivisionByZero`] or a [`Self::NonFinite`]
    /// result. A condition with such a part is unknown rather than broken, see the
    /// [module documentation](self).
    pub const fn is_unknown(&self) -> bool {
        matches!(
            self,
            Self::Undefined(_) | Self::DivisionByZero(_) | Self::NonFinite(_)
        )
    }
}

/// The result of checking a condition: `None` if it has the wanted truth value, and an
/// error for which [`EvalError::is_unknown`] holds if its truth value is unknown.
pub type Verdict = Result<Option<Unmet>, EvalError>;

/// Evaluates conditions in one state.
pub struct Evaluator<'s, 'a> {
    /// The indexed domain and problem.
    pub model: &'s Model<'a>,
    /// The state to evaluate in.
    pub state: &'s State,
    /// The relative tolerance of numeric comparisons; see [`numeric`].
    pub tolerance: f64,
    /// Records what the evaluation reads, for temporal plans; `None` otherwise.
    pub observer: Option<&'s RefCell<Observer>>,
}

/// What an evaluation read, for the interference and invariant checks of temporal plans.
#[derive(Debug, Default)]
pub struct Observer {
    /// The ground atoms read.
    pub atoms: BTreeSet<Atom>,
    /// The numeric fluents read.
    pub fluents: BTreeSet<Atom>,
    /// `left - right` of every comparison, in evaluation order.
    pub differences: Vec<f64>,
    /// The fluents that change continuously while the condition is checked.
    pub moving: BTreeSet<Atom>,
    /// `true` once a comparison is not linear in the `moving` fluents.
    pub nonlinear: bool,
}

impl Evaluator<'_, '_> {
    /// Checks that every item of a precondition or goal list holds.
    pub fn preconditions(&self, list: &PreconditionGoalDescriptions, scope: &mut Scope) -> Verdict {
        let mut all = All::default();
        for item in list {
            let verdict = match item {
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(gd)) => {
                    self.holds(gd, scope, true)
                }
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Preference(
                    _,
                )) => return Err(EvalError::Unsupported(Construct::Preferences)),
                PreconditionGoalDescription::Forall(variables, inner) => {
                    let mut instances = All::default();
                    self.each_binding(variables, scope, |scope| {
                        instances.record(self.preconditions(inner, scope))
                    })?;
                    instances.into_verdict()
                }
            };
            all.record(verdict)?;
        }

        all.into_verdict()
    }

    /// Checks that `gd` has the truth value `want`.
    pub fn holds(&self, gd: &GoalDescription, scope: &mut Scope, want: bool) -> Verdict {
        match gd {
            GoalDescription::AtomicFormula(formula)
            | GoalDescription::Literal(Literal::AtomicFormula(formula)) => {
                self.atomic(formula, scope, want)
            }
            GoalDescription::Literal(Literal::NotAtomicFormula(formula)) => {
                self.atomic(formula, scope, !want)
            }
            GoalDescription::Not(inner) => self.holds(inner, scope, !want),
            GoalDescription::And(items) if want => self.every(items, scope, true),
            GoalDescription::And(items) => self.some(gd, items, scope, false),
            GoalDescription::Or(items) if want => self.some(gd, items, scope, true),
            GoalDescription::Or(items) => self.every(items, scope, false),
            GoalDescription::Imply(premise, conclusion) => {
                self.implication(gd, premise, conclusion, scope, want)
            }
            GoalDescription::Exists(variables, inner) => {
                self.quantified(gd, variables, inner, scope, want, !want)
            }
            GoalDescription::ForAll(variables, inner) => {
                self.quantified(gd, variables, inner, scope, want, want)
            }
            GoalDescription::FluentComparison(comparison) => {
                self.comparison(comparison, scope, want)
            }
        }
    }

    /// Checks that every item has the truth value `want`; blames the first that does not.
    fn every(&self, items: &[GoalDescription], scope: &mut Scope, want: bool) -> Verdict {
        let mut all = All::default();
        for item in items {
            all.record(self.holds(item, scope, want))?;
        }

        all.into_verdict()
    }

    /// Checks that some item has the truth value `want`; blames all of them if none does.
    fn some(
        &self,
        gd: &GoalDescription,
        items: &[GoalDescription],
        scope: &mut Scope,
        want: bool,
    ) -> Verdict {
        let mut failures = Failures::default();
        for item in items {
            failures.record(self.holds(item, scope, want))?;
        }

        failures.into_verdict(|| condition(gd, scope, want))
    }

    /// Checks `(imply premise conclusion)`, which is `(or (not premise) conclusion)`.
    fn implication(
        &self,
        gd: &GoalDescription,
        premise: &GoalDescription,
        conclusion: &GoalDescription,
        scope: &mut Scope,
        want: bool,
    ) -> Verdict {
        if !want {
            // False only if the premise holds and the conclusion does not.
            let mut all = All::default();
            all.record(self.holds(premise, scope, true))?;
            all.record(self.holds(conclusion, scope, false))?;
            return all.into_verdict();
        }

        let mut failures = Failures::default();
        failures.record(self.holds(premise, scope, false))?;
        failures.record(self.holds(conclusion, scope, true))?;

        failures.into_verdict(|| condition(gd, scope, want))
    }

    /// Checks a quantified formula. With `every`, each binding of `variables` must give
    /// `inner` the truth value `want`; without, one binding must.
    fn quantified(
        &self,
        gd: &GoalDescription,
        variables: &TypedVariables,
        inner: &GoalDescription,
        scope: &mut Scope,
        want: bool,
        every: bool,
    ) -> Verdict {
        if every {
            let mut all = All::default();
            self.each_binding(variables, scope, |scope| {
                all.record(self.holds(inner, scope, want))
            })?;
            return all.into_verdict();
        }

        let mut failures = Failures::default();
        self.each_binding(variables, scope, |scope| {
            failures.record(self.holds(inner, scope, want))
        })?;

        failures.into_verdict(|| condition(gd, scope, want))
    }

    /// Calls `f` once per binding of `variables` to objects of their types, with the
    /// bindings pushed onto `scope`. Calls it never if a type has no objects, and once with
    /// `scope` unchanged if `variables` is empty. `scope` is restored afterwards.
    pub fn each_binding(
        &self,
        variables: &TypedVariables,
        scope: &mut Scope,
        mut f: impl FnMut(&mut Scope) -> Result<(), EvalError>,
    ) -> Result<(), EvalError> {
        let candidates: Vec<(String, Vec<&str>)> = variables
            .iter()
            .map(|v| (lower(v.value()), self.model.universe.objects_of(v.type_())))
            .collect();
        if candidates.iter().any(|(_, objects)| objects.is_empty()) {
            return Ok(());
        }

        let base = scope.len();
        let mut odometer: Vec<usize> = alloc::vec![0; candidates.len()];
        loop {
            scope.truncate(base);
            let indices = candidates.iter().zip(&odometer);
            scope.extend(indices.filter_map(|((name, objects), &index)| {
                let object = objects.get(index)?;
                Some((name.clone(), (*object).to_string()))
            }));

            let result = f(scope);
            scope.truncate(base);
            result?;
            if !advance(&mut odometer, &candidates) {
                return Ok(());
            }
        }
    }

    /// Checks an atomic formula against `want`.
    fn atomic(&self, formula: &AtomicFormula<Term>, scope: &Scope, want: bool) -> Verdict {
        Ok(match formula {
            AtomicFormula::Equality(equality) => {
                if let Some(comparison) = numeric::numeric_equality(equality) {
                    return self.comparison(&comparison, scope, want);
                }

                let left = resolve(equality.first(), scope);
                let right = resolve(equality.second(), scope);
                ((left == right) != want).then_some(Unmet::Equality {
                    left,
                    right,
                    expected: want,
                })
            }
            AtomicFormula::Predicate(predicate) => {
                let atom = ground(predicate, scope);
                if let Some(observer) = self.observer {
                    observer.borrow_mut().atoms.insert(atom.clone());
                }

                let derived = self.model.derived.contains(&atom.symbol);
                let set = if derived {
                    &self.state.derived
                } else {
                    &self.state.facts
                };
                (set.contains(&atom) != want).then_some(Unmet::Atom {
                    atom,
                    expected: want,
                    derived,
                })
            }
        })
    }
}

/// Renders `gd` for a [`Unmet::NoneOf`], negated if it was wanted false.
fn condition(gd: &GoalDescription, scope: &Scope, want: bool) -> String {
    let text = render::goal(gd, scope);
    if want {
        text
    } else {
        alloc::format!("(not {text})")
    }
}

/// Combines the verdicts of the parts of a conjunction-like condition: the first
/// definite failure, else the first unknown part, else success.
#[derive(Default)]
struct All {
    /// The first part that definitely fails.
    failure: Option<Unmet>,
    /// The first part whose truth value is unknown.
    unknown: Option<EvalError>,
}

impl All {
    /// Records the verdict of one part.
    ///
    /// # Errors
    ///
    /// A fatal [`EvalError`] of the part, one for which [`EvalError::is_unknown`] is false.
    fn record(&mut self, verdict: Verdict) -> Result<(), EvalError> {
        match verdict {
            Ok(None) => {}
            Ok(Some(unmet)) => {
                self.failure.get_or_insert(unmet);
            }
            Err(error) => record_unknown(&mut self.unknown, error)?,
        }

        Ok(())
    }

    /// Returns the verdict of the whole condition.
    fn into_verdict(self) -> Verdict {
        match (self.failure, self.unknown) {
            (Some(unmet), _) => Ok(Some(unmet)),
            (None, Some(error)) => Err(error),
            (None, None) => Ok(None),
        }
    }
}

/// Collects the failing alternatives of a disjunction-like condition: success if one
/// definitely holds, else the first unknown alternative, else all failures.
#[derive(Default)]
struct Failures {
    /// `true` once an alternative holds.
    satisfied: bool,
    /// The first alternative whose truth value is unknown.
    unknown: Option<EvalError>,
    /// The first [`MAX_ALTERNATIVES`] failures.
    kept: Vec<Unmet>,
    /// The number of failures beyond those kept.
    omitted: usize,
}

impl Failures {
    /// Records the verdict of one alternative.
    ///
    /// # Errors
    ///
    /// A fatal [`EvalError`] of the alternative, one for which [`EvalError::is_unknown`]
    /// is false.
    fn record(&mut self, verdict: Verdict) -> Result<(), EvalError> {
        match verdict {
            Ok(None) => self.satisfied = true,
            Ok(Some(unmet)) if self.kept.len() < MAX_ALTERNATIVES => self.kept.push(unmet),
            Ok(Some(_)) => self.omitted += 1,
            Err(error) => record_unknown(&mut self.unknown, error)?,
        }

        Ok(())
    }

    /// Returns `None` if an alternative held, else the first unknown alternative's error,
    /// and else a [`Unmet::NoneOf`] over the failures, with the condition text that
    /// `condition` renders.
    fn into_verdict(self, condition: impl FnOnce() -> String) -> Verdict {
        if self.satisfied {
            return Ok(None);
        }

        if let Some(error) = self.unknown {
            return Err(error);
        }

        Ok(Some(Unmet::NoneOf {
            condition: condition(),
            alternatives: self.kept,
            omitted: self.omitted,
        }))
    }
}

/// Keeps `error` in `unknown` unless an earlier unknown part is already there.
///
/// # Errors
///
/// `error` itself if it is fatal, that is, if [`EvalError::is_unknown`] is false.
fn record_unknown(unknown: &mut Option<EvalError>, error: EvalError) -> Result<(), EvalError> {
    if !error.is_unknown() {
        return Err(error);
    }

    unknown.get_or_insert(error);
    Ok(())
}

/// Moves `odometer` to the next combination of candidate indices. Returns `false` after
/// the last one.
fn advance(odometer: &mut [usize], candidates: &[(String, Vec<&str>)]) -> bool {
    for (index, (_, objects)) in odometer.iter_mut().zip(candidates).rev() {
        *index += 1;
        if *index < objects.len() {
            return true;
        }

        *index = 0;
    }

    false
}

/// Returns the object a term denotes in `scope`.
///
/// The static check guarantees that every variable is bound and that no term is a
/// function; should either ever slip through, the term resolves to text that names no
/// object (`?x`), so atoms over it are false rather than the validator panicking.
pub fn resolve(term: &Term, scope: &Scope) -> String {
    match term {
        Term::Name(name) => lower(name),
        Term::Variable(_) | Term::Function(_) => render::term(term, scope),
    }
}

/// Grounds an atomic formula over terms in `scope`.
pub fn ground(atom: &PredicateAtomicFormula<Term>, scope: &Scope) -> Atom {
    let arguments = atom.values().iter().map(|t| resolve(t, scope)).collect();
    Atom::lowered(lower(atom.predicate()), arguments)
}
