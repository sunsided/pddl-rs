//! Derived predicates: stratification of the rules and their least fixpoint in a state.
//!
//! A rule may read derived atoms positively from its own stratum and below, but under
//! negation only from strata below. Within a stratum the rules are then monotone, so
//! applying them until nothing changes reaches the least fixpoint, stratum by stratum.
//! This is the semantics of PDDL 2.2 and of Fast Downward's axioms.
//!
//! ```mermaid
//! flowchart LR
//!     facts["basic facts"] --> s0["stratum 0 fixpoint"]
//!     s0 --> s1["stratum 1 fixpoint"]
//!     s1 --> sn["..."]
//!     sn --> derived["derived atoms"]
//! ```

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;

use pddl::{AtomicFormula, GoalDescription, Literal};

use crate::atom::{lower, Atom};
use crate::eval::{EvalError, Evaluator, Scope};
use crate::model::Model;
use crate::report::{Construct, Unsupported};
use crate::state::State;

/// Assigns every rule its stratum and sorts the rules by it.
///
/// # Errors
///
/// [`Construct::UnstratifiedDerivedPredicates`] if a derived predicate depends on its
/// own negation, directly or through other rules.
pub fn stratify(model: &mut Model<'_>) -> Result<(), Unsupported> {
    let dependencies: Vec<Vec<(String, bool)>> = model
        .rules
        .iter()
        .map(|rule| {
            let mut found = Vec::new();
            occurrences(rule.body, false, &mut found);
            found.retain(|(name, _)| model.derived.contains(name));
            found
        })
        .collect();

    let mut level: BTreeMap<&str, usize> = model
        .rules
        .iter()
        .map(|rule| (rule.predicate.as_str(), 0))
        .collect();
    // A stratification needs at most one level per derived predicate.
    let limit = level.len();
    let mut changed = true;
    while changed {
        changed = false;
        for (rule, deps) in model.rules.iter().zip(&dependencies) {
            for (name, negated) in deps {
                let needed = level.get(name.as_str()).copied().unwrap_or(0) + usize::from(*negated);
                let own = level.entry(rule.predicate.as_str()).or_insert(0);
                if *own < needed {
                    if needed > limit {
                        let location = alloc::format!("derived predicate `{}`", rule.predicate);
                        let construct = Construct::UnstratifiedDerivedPredicates;
                        return Err(Unsupported::new(construct, location));
                    }

                    *own = needed;
                    changed = true;
                }
            }
        }
    }

    let strata: Vec<usize> = model
        .rules
        .iter()
        .map(|rule| level.get(rule.predicate.as_str()).copied().unwrap_or(0))
        .collect();
    for (rule, stratum) in model.rules.iter_mut().zip(strata) {
        rule.stratum = stratum;
    }

    model.rules.sort_by_key(|rule| rule.stratum);
    Ok(())
}

/// Collects the predicates `gd` reads, each with `true` if it reads it under negation.
/// An implication's premise counts as negated.
fn occurrences(gd: &GoalDescription, negated: bool, out: &mut Vec<(String, bool)>) {
    match gd {
        GoalDescription::AtomicFormula(formula)
        | GoalDescription::Literal(Literal::AtomicFormula(formula)) => {
            predicate(formula, negated, out);
        }
        GoalDescription::Literal(Literal::NotAtomicFormula(formula)) => {
            predicate(formula, !negated, out);
        }
        GoalDescription::And(items) | GoalDescription::Or(items) => {
            for item in items {
                occurrences(item, negated, out);
            }
        }
        GoalDescription::Not(inner) => occurrences(inner, !negated, out),
        GoalDescription::Imply(premise, conclusion) => {
            occurrences(premise, !negated, out);
            occurrences(conclusion, negated, out);
        }
        GoalDescription::Exists(_, inner) | GoalDescription::ForAll(_, inner) => {
            occurrences(inner, negated, out);
        }
        GoalDescription::FluentComparison(_) => {}
    }
}

/// Records the predicate of `formula`, if it has one.
fn predicate<T>(formula: &AtomicFormula<T>, negated: bool, out: &mut Vec<(String, bool)>) {
    if let AtomicFormula::Predicate(atom) = formula {
        out.push((lower(atom.predicate()), negated));
    }
}

/// Recomputes `state.derived` from `state.facts`.
///
/// Rules are applied stratum by stratum until no rule adds an atom. Each round evaluates
/// every rule for every binding of its parameters, so a rule of arity `k` over `n`
/// objects costs `n^k` evaluations per round.
///
/// # Errors
///
/// The first [`EvalError`] of a rule body.
pub fn close(model: &Model<'_>, state: &mut State) -> Result<(), EvalError> {
    state.derived.clear();
    let mut rest = model.rules.as_slice();
    while let Some(first) = rest.first() {
        let size = rest
            .iter()
            .take_while(|rule| rule.stratum == first.stratum)
            .count();
        let (stratum, later) = rest.split_at(size);
        loop {
            let new = round(model, state, stratum)?;
            if new.is_empty() {
                break;
            }

            state.derived.extend(new);
        }

        rest = later;
    }

    Ok(())
}

/// Applies the rules of one stratum once and returns the atoms they add.
fn round(
    model: &Model<'_>,
    state: &State,
    rules: &[crate::model::Rule<'_>],
) -> Result<BTreeSet<Atom>, EvalError> {
    let evaluator = Evaluator { model, state };
    let mut new = BTreeSet::new();
    for rule in rules {
        let mut scope = Scope::new();
        evaluator.each_binding(rule.parameters, &mut scope, |scope| {
            let arguments = scope.iter().map(|(_, object)| object.clone()).collect();
            let atom = Atom::lowered(rule.predicate.clone(), arguments);
            if !state.derived.contains(&atom)
                && !new.contains(&atom)
                && evaluator.holds(rule.body, scope, true)?.is_none()
            {
                new.insert(atom);
            }

            Ok(())
        })?;
    }

    Ok(new)
}
