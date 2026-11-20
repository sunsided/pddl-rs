//! Effects: collected in the state before a step, then applied at once.
//!
//! Every `when` condition is evaluated in the state before the step, as is every bound of
//! a `forall`. The new state is the old one minus the deleted atoms plus the added ones,
//! so an atom that a step both deletes and adds stays true, as in VAL and PDDL 2.1.

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use pddl::{AtomicFormula, ConditionalEffect, EffectCondition, Effects, PrimitiveEffect, Term};

use crate::atom::Atom;
use crate::eval::{ground, EvalError, Evaluator, Scope};
use crate::report::{Construct, StepTrace};
use crate::state::State;

/// The ground changes of one step, before they are applied.
#[derive(Debug, Default)]
pub struct Changes {
    /// Atoms the step makes true.
    add: BTreeSet<Atom>,
    /// Atoms the step makes false, unless it also adds them.
    delete: BTreeSet<Atom>,
}

impl Changes {
    /// Collects the changes of `effects` with `scope` bound, evaluating conditions with
    /// `evaluator`.
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
                ConditionalEffect::Effect(primitive) => self.primitive(primitive, scope)?,
                ConditionalEffect::Forall(forall) => {
                    evaluator.each_binding(&forall.variables, scope, |scope| {
                        self.collect(evaluator, &forall.effects, scope)
                    })?;
                }
                ConditionalEffect::When(when) => {
                    if evaluator.holds(&when.condition, scope, true)?.is_none() {
                        self.conditional(&when.effect, scope)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Collects the effects of a `when` whose condition holds.
    fn conditional(&mut self, effect: &EffectCondition, scope: &Scope) -> Result<(), EvalError> {
        match effect {
            EffectCondition::Single(primitive) => self.primitive(primitive, scope),
            EffectCondition::All(primitives) => primitives
                .iter()
                .try_for_each(|primitive| self.primitive(primitive, scope)),
        }
    }

    /// Collects one primitive effect.
    fn primitive(&mut self, effect: &PrimitiveEffect, scope: &Scope) -> Result<(), EvalError> {
        match effect {
            PrimitiveEffect::AtomicFormula(formula) => {
                self.add.insert(settable(formula, scope)?);
            }
            PrimitiveEffect::NotAtomicFormula(formula) => {
                self.delete.insert(settable(formula, scope)?);
            }
            PrimitiveEffect::AssignNumericFluent(..) => {
                return Err(EvalError::Unsupported(Construct::NumericFluents));
            }
            PrimitiveEffect::AssignObjectFluent(..) => {
                return Err(EvalError::Unsupported(Construct::ObjectFluents));
            }
        }

        Ok(())
    }

    /// Applies the changes to `state`: deletes first, then adds. Returns the atoms whose
    /// truth value changed.
    pub fn apply(self, state: &mut State) -> StepTrace {
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

        StepTrace {
            step: None,
            added,
            deleted,
        }
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
