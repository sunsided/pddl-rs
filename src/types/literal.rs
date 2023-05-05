//! Contains literals via the [`Literal`] type.

use crate::types::AtomicFormula;

/// An [`AtomicFormula`] or its negated value.
///
/// ## Usage
/// Used by [`GoalDefinition`](crate::GoalDefinition) and [`InitElement`](crate::InitElement).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal<T> {
    AtomicFormula(AtomicFormula<T>),
    NotAtomicFormula(AtomicFormula<T>),
}

impl<'a, T> Literal<T> {
    pub const fn new(atomic_formula: AtomicFormula<T>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    pub const fn new_not(atomic_formula: AtomicFormula<T>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }

    pub const fn is_negated(&self) -> bool {
        matches!(self, Self::NotAtomicFormula(..))
    }
}

impl<'a, T> From<AtomicFormula<T>> for Literal<T> {
    fn from(value: AtomicFormula<T>) -> Self {
        Literal::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{atomic_formula, parse_term, Span};
    use crate::Term;

    #[test]
    fn from_works() {
        let input = "(= x y)";
        let (_, effect) = atomic_formula(parse_term)(Span::new(input)).unwrap();

        let literal: Literal<Term> = effect.into();
        assert_eq!(
            literal,
            Literal::AtomicFormula(AtomicFormula::new_equality(
                Term::new_name("x".into()),
                Term::new_name("y".into()),
            ))
        )
    }
}
