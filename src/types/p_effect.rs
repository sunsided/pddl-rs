//! Contains p-effects.

use crate::types::{AtomicFormula, Term};

/// A p-effect.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PEffect<'a> {
    AtomicFormula(AtomicFormula<'a, Term<'a>>),
    NotAtomicFormula(AtomicFormula<'a, Term<'a>>),
    // TODO: AssignNumericFluent,
    // TODO: AssignObjectFluent
}

impl<'a> PEffect<'a> {
    pub const fn new(atomic_formula: AtomicFormula<'a, Term<'a>>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    pub const fn new_not(atomic_formula: AtomicFormula<'a, Term<'a>>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }
}
