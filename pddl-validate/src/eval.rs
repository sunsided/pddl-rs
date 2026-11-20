//! Evaluation of conditions in a state, with an explanation when they fail.
//!
//! [`Evaluator::holds`] checks a goal description against a wanted truth value and,
//! when it does not have it, returns the most specific part to blame ([`Unmet`]).
//! Negation flips the wanted value instead of evaluating and inverting, so a failing
//! `(not (or a b))` blames `a` or `b` rather than the whole disjunction.
//!
//! Evaluation never short-circuits: every part of a condition is evaluated, so whether
//! it fails with an [`EvalError`] does not depend on the order of its parts.
//! [`numeric`] evaluates numeric expressions and comparisons.

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

/// The result of checking a condition: `None` if it has the wanted truth value.
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
        let mut first = None;
        for item in list {
            let unmet = match item {
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(gd)) => {
                    self.holds(gd, scope, true)?
                }
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Preference(
                    _,
                )) => return Err(EvalError::Unsupported(Construct::Preferences)),
                PreconditionGoalDescription::Forall(variables, inner) => {
                    let mut failure = None;
                    self.each_binding(variables, scope, |scope| {
                        let unmet = self.preconditions(inner, scope)?;
                        failure = failure.take().or(unmet);
                        Ok(())
                    })?;
                    failure
                }
            };
            first = first.or(unmet);
        }

        Ok(first)
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
        let mut first = None;
        for item in items {
            let unmet = self.holds(item, scope, want)?;
            first = first.or(unmet);
        }

        Ok(first)
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
            failures.record(self.holds(item, scope, want)?);
        }

        Ok(failures.into_unmet(|| condition(gd, scope, want)))
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
            let premise = self.holds(premise, scope, true)?;
            let conclusion = self.holds(conclusion, scope, false)?;
            return Ok(premise.or(conclusion));
        }

        let mut failures = Failures::default();
        failures.record(self.holds(premise, scope, false)?);
        failures.record(self.holds(conclusion, scope, true)?);

        Ok(failures.into_unmet(|| condition(gd, scope, want)))
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
        let mut first = None;
        let mut failures = Failures::default();
        self.each_binding(variables, scope, |scope| {
            let unmet = self.holds(inner, scope, want)?;
            if every {
                first = first.take().or(unmet);
            } else {
                failures.record(unmet);
            }

            Ok(())
        })?;

        if every {
            Ok(first)
        } else {
            Ok(failures.into_unmet(|| condition(gd, scope, want)))
        }
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
            for ((name, objects), &index) in candidates.iter().zip(&odometer) {
                if let Some(object) = objects.get(index) {
                    scope.push((name.clone(), (*object).to_string()));
                }
            }

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

/// Collects the failing alternatives of a disjunction-like condition.
#[derive(Default)]
struct Failures {
    /// `true` once an alternative holds.
    satisfied: bool,
    /// The first [`MAX_ALTERNATIVES`] failures.
    kept: Vec<Unmet>,
    /// The number of failures beyond those kept.
    omitted: usize,
}

impl Failures {
    /// Records the verdict of one alternative.
    fn record(&mut self, unmet: Option<Unmet>) {
        match unmet {
            None => self.satisfied = true,
            Some(unmet) if self.kept.len() < MAX_ALTERNATIVES => self.kept.push(unmet),
            Some(_) => self.omitted += 1,
        }
    }

    /// Returns `None` if an alternative held, and else a [`Unmet::NoneOf`] over the
    /// failures, with the condition text that `condition` renders.
    fn into_unmet(self, condition: impl FnOnce() -> String) -> Option<Unmet> {
        (!self.satisfied).then(|| Unmet::NoneOf {
            condition: condition(),
            alternatives: self.kept,
            omitted: self.omitted,
        })
    }
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
