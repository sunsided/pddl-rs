//! Effects: collected in the state before a step, then applied at once.
//!
//! Every `when` condition, every bound of a `forall` and every right-hand side of a
//! numeric update is evaluated in the state before the step. The new state is the old
//! one minus the deleted atoms plus the added ones, so an atom that a step both deletes
//! and adds stays true, as in VAL and PDDL 2.1.
//!
//! Numeric updates of one fluent combine if they are all `increase` and `decrease`: their
//! deltas add up, so `(increase (total-cost) 1)` twice adds 2. Any other pair of updates
//! of one fluent, such as two `assign`s, is an [`EvalError::ConflictingUpdates`]: their
//! result would depend on an order that PDDL does not define.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::vec::Vec;

use pddl::{
    AssignOp, AtomicFormula, ConditionalEffect, EffectCondition, Effects, PrimitiveEffect, Term,
};

use crate::atom::Atom;
use crate::eval::numeric::{fluent_atom, numeric_assignment};
use crate::eval::{ground, EvalError, Evaluator, Scope};
use crate::report::{Construct, FluentChange, StepTrace};
use crate::state::State;

/// The ground changes of one step, before they are applied.
#[derive(Debug, Default)]
pub struct Changes {
    /// Atoms the step makes true.
    add: BTreeSet<Atom>,
    /// Atoms the step makes false, unless it also adds them.
    delete: BTreeSet<Atom>,
    /// The numeric updates per fluent, with their right-hand sides evaluated.
    updates: BTreeMap<Atom, Vec<(AssignOp, f64)>>,
}

impl Changes {
    /// Collects the changes of `effects` with `scope` bound, evaluating conditions and
    /// right-hand sides with `evaluator`.
    ///
    /// # Errors
    ///
    /// The first [`EvalError`] of a `when` condition or effect.
    pub fn collect(
        &mut self,
        evaluator: &Evaluator<'_, '_>,
        effects: &Effects,
        scope: &mut Scope,
    ) -> Result<(), EvalError> {
        for effect in effects {
            match effect {
                ConditionalEffect::Effect(primitive) => {
                    self.primitive(evaluator, primitive, scope)?;
                }
                ConditionalEffect::Forall(forall) => {
                    evaluator.each_binding(&forall.variables, scope, |scope| {
                        self.collect(evaluator, &forall.effects, scope)
                    })?;
                }
                ConditionalEffect::When(when) => {
                    if evaluator.holds(&when.condition, scope, true)?.is_none() {
                        let primitives = match &when.effect {
                            EffectCondition::Single(primitive) => core::slice::from_ref(primitive),
                            EffectCondition::All(primitives) => primitives.as_slice(),
                        };
                        for primitive in primitives {
                            self.primitive(evaluator, primitive, scope)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Collects one primitive effect.
    fn primitive(
        &mut self,
        evaluator: &Evaluator<'_, '_>,
        effect: &PrimitiveEffect,
        scope: &Scope,
    ) -> Result<(), EvalError> {
        match effect {
            PrimitiveEffect::AtomicFormula(formula) => {
                self.add.insert(settable(formula, scope)?);
            }
            PrimitiveEffect::NotAtomicFormula(formula) => {
                self.delete.insert(settable(formula, scope)?);
            }
            PrimitiveEffect::AssignNumericFluent(op, head, value) => {
                let value = evaluator.value(value, scope)?;
                let fluent = fluent_atom(head, scope);
                self.updates
                    .entry(fluent)
                    .or_default()
                    .push((op.clone(), value));
            }
            PrimitiveEffect::AssignObjectFluent(..) => {
                let Some((head, value)) = numeric_assignment(effect) else {
                    return Err(EvalError::Unsupported(Construct::ObjectFluents));
                };

                let value = evaluator.value(&value, scope)?;
                let fluent = fluent_atom(&head, scope);
                self.updates
                    .entry(fluent)
                    .or_default()
                    .push((AssignOp::Assign, value));
            }
        }

        Ok(())
    }

    /// Applies the changes to `state`: deletes first, then adds, then numeric updates.
    /// Returns what changed.
    ///
    /// # Errors
    ///
    /// [`EvalError::ConflictingUpdates`] for updates of one fluent that do not combine,
    /// [`EvalError::Undefined`] for a relative update of a fluent without a value, and
    /// [`EvalError::DivisionByZero`] or [`EvalError::NonFinite`] for its arithmetic.
    /// `state` is unchanged on error.
    pub fn apply(self, state: &mut State) -> Result<StepTrace, EvalError> {
        let numeric = numeric_changes(&self.updates, state)?;
        let deleted: Vec<Atom> = self
            .delete
            .iter()
            .filter(|atom| state.facts.contains(*atom) && !self.add.contains(*atom))
            .cloned()
            .collect();
        let added: Vec<Atom> = self
            .add
            .iter()
            .filter(|atom| !state.facts.contains(*atom))
            .cloned()
            .collect();
        for atom in &deleted {
            state.facts.remove(atom);
        }

        state.facts.extend(added.iter().cloned());
        for change in &numeric {
            state.fluents.insert(change.fluent.clone(), change.after);
        }

        Ok(StepTrace {
            step: None,
            added,
            deleted,
            numeric,
        })
    }
}

/// Computes the new value of every updated fluent.
fn numeric_changes(
    updates: &BTreeMap<Atom, Vec<(AssignOp, f64)>>,
    state: &State,
) -> Result<Vec<FluentChange>, EvalError> {
    let mut changes = Vec::with_capacity(updates.len());
    for (fluent, ops) in updates {
        let before = state.fluents.get(fluent).copied();
        let after = match ops.as_slice() {
            [(op, value)] => update(op, before, *value, fluent)?,
            _ if ops
                .iter()
                .all(|(op, _)| matches!(op, AssignOp::Increase | AssignOp::Decrease)) =>
            {
                let delta = ops.iter().fold(0.0, |sum, (op, value)| match op {
                    AssignOp::Decrease => sum - value,
                    _ => sum + value,
                });
                update(&AssignOp::Increase, before, delta, fluent)?
            }
            _ => return Err(EvalError::ConflictingUpdates(fluent.clone())),
        };
        changes.push(FluentChange {
            fluent: fluent.clone(),
            before,
            after,
        });
    }

    Ok(changes)
}

/// Applies one update to the value `before` of `fluent`.
fn update(op: &AssignOp, before: Option<f64>, value: f64, fluent: &Atom) -> Result<f64, EvalError> {
    let old = || before.ok_or_else(|| EvalError::Undefined(fluent.clone()));
    let after = match op {
        AssignOp::Assign => value,
        AssignOp::Increase => old()? + value,
        AssignOp::Decrease => old()? - value,
        AssignOp::ScaleUp => old()? * value,
        AssignOp::ScaleDown if value == 0.0 => {
            return Err(EvalError::DivisionByZero(alloc::format!(
                "(scale-down {fluent} 0)"
            )));
        }
        AssignOp::ScaleDown => old()? / value,
    };

    if after.is_finite() {
        Ok(after)
    } else {
        Err(EvalError::NonFinite(alloc::format!("{fluent}")))
    }
}

/// Grounds the atom an add or delete effect sets.
///
/// # Errors
///
/// An equality, which the static check rejects; reported as unsupported should it ever
/// get through.
fn settable(formula: &AtomicFormula<Term>, scope: &Scope) -> Result<Atom, EvalError> {
    match formula {
        AtomicFormula::Predicate(atom) => Ok(ground(atom, scope)),
        AtomicFormula::Equality(_) => Err(EvalError::Unsupported(Construct::EqualityEffects)),
    }
}
