//! Contains literals.

use crate::types::AtomicFormula;

/// An atomic formula.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal<'a, T> {
    Is(AtomicFormula<'a, T>),
    Not(AtomicFormula<'a, T>),
}

impl<'a, T> Literal<'a, T> {
    pub const fn new(atomic_formula: AtomicFormula<'a, T>) -> Self {
        Self::Is(atomic_formula)
    }

    pub const fn new_not(atomic_formula: AtomicFormula<'a, T>) -> Self {
        Self::Not(atomic_formula)
    }
}
