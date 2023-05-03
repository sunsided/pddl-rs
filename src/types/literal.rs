//! Contains literals via the [`Literal`] type.

use crate::types::AtomicFormula;

/// An [`AtomicFormula`] or its negated value.
///
/// ## Usage
/// Used by [`GoalDefinition`](crate::GoalDefinition) and [`InitElement`](crate::InitElement).
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

    pub const fn is_negated(&self) -> bool {
        matches!(self, Self::NotAtomicFormula(..))
    }
}
