//! Contains literals.

use crate::types::domain::AtomicFormula;

/// An atomic formula.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal<'a, T> {
    AtomicFormula(AtomicFormula<'a, T>),
    NotAtomicFormula(AtomicFormula<'a, T>),
}

impl<'a, T> Literal<'a, T> {
    pub const fn new(atomic_formula: AtomicFormula<'a, T>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    pub const fn new_not(atomic_formula: AtomicFormula<'a, T>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }
}
