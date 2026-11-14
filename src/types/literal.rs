//! Contains literals via the [`Literal`] type.

use crate::types::AtomicFormula;

/// An [`AtomicFormula`] or its negated value.
///
/// ## Usage
/// Used by [`GoalDefinition`](crate::GoalDefinition) and [`InitElement`](crate::InitElement).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal<T> {
    /// A positive literal, `<atomic formula(t)>`.
    AtomicFormula(AtomicFormula<T>),
    /// A negated literal, `(not <atomic formula(t)>)`.
    NotAtomicFormula(AtomicFormula<T>),
}

impl<T> Literal<T> {
    /// Creates a positive literal.
    pub const fn new(atomic_formula: AtomicFormula<T>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    /// Creates a negated literal `(not <atomic formula(t)>)`.
    pub const fn new_not(atomic_formula: AtomicFormula<T>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }

    /// Returns `true` if this is a negated literal.
    pub const fn is_negated(&self) -> bool {
        matches!(self, Self::NotAtomicFormula(..))
    }
}

impl<T> From<AtomicFormula<T>> for Literal<T> {
    fn from(value: AtomicFormula<T>) -> Self {
        Literal::new(value)
    }
}

#[cfg(all(test, feature = "parser"))]
mod tests {
    use super::*;
    use crate::parsers::{atomic_formula, parse_one, parse_term};
    use crate::Term;

    #[test]
    fn from_works() {
        let input = "(= x y)";
        let effect = parse_one(input, |n| atomic_formula(n, parse_term)).unwrap();

        let literal: Literal<Term> = effect.into();
        assert_eq!(
            literal,
            Literal::AtomicFormula(AtomicFormula::equality(
                Term::new_name("x".into()),
                Term::new_name("y".into()),
            ))
        )
    }
}
