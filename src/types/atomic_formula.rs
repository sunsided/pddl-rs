//! Contains atomic formulae via the [`AtomicFormula`] type.

use crate::types::Predicate;
use std::ops::Deref;

/// An atomic formula.
///
/// ## Usage
/// Used by [`Literal`](crate::Literal), [`GoalDefinition`](crate::GoalDefinition) and
/// [`PEffect`](crate::PEffect).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AtomicFormula<T> {
    Equality(EqualityAtomicFormula<T>),
    Predicate(PredicateAtomicFormula<T>),
}

impl<'a, T> AtomicFormula<T> {
    pub const fn new_equality(first: T, second: T) -> Self {
        Self::Equality(EqualityAtomicFormula::new(first, second))
    }

    pub fn new_predicate<V: IntoIterator<Item = T>>(predicate: Predicate, values: V) -> Self {
        Self::Predicate(PredicateAtomicFormula::new(
            predicate,
            values.into_iter().collect(),
        ))
    }
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct EqualityAtomicFormula<T> {
    first: T,
    second: T,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct PredicateAtomicFormula<T> {
    predicate: Predicate,
    values: Vec<T>,
}

impl<T> EqualityAtomicFormula<T> {
    pub const fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    /// Gets a reference to the first element.
    pub const fn first(&self) -> &T {
        &self.first
    }

    /// Gets a reference to the second element.
    pub const fn second(&self) -> &T {
        &self.second
    }
}

impl<T> PredicateAtomicFormula<T> {
    pub const fn new(predicate: Predicate, values: Vec<T>) -> Self {
        Self { predicate, values }
    }

    /// Returns the predicate.
    pub const fn predicate(&self) -> &Predicate {
        &self.predicate
    }

    /// Gets a reference to the values.
    pub fn values(&self) -> &[T] {
        self.values.as_slice()
    }
}

impl<'a, T> From<EqualityAtomicFormula<T>> for AtomicFormula<T> {
    fn from(value: EqualityAtomicFormula<T>) -> Self {
        AtomicFormula::Equality(value)
    }
}

impl<'a, T> From<PredicateAtomicFormula<T>> for AtomicFormula<T> {
    fn from(value: PredicateAtomicFormula<T>) -> Self {
        AtomicFormula::Predicate(value)
    }
}

impl<T> From<(T, T)> for EqualityAtomicFormula<T> {
    fn from(value: (T, T)) -> Self {
        EqualityAtomicFormula::new(value.0, value.1)
    }
}

impl<'a, T> From<(Predicate, Vec<T>)> for PredicateAtomicFormula<T> {
    fn from(value: (Predicate, Vec<T>)) -> Self {
        PredicateAtomicFormula::new(value.0, value.1)
    }
}

impl<'a, T> Deref for PredicateAtomicFormula<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.values()
    }
}
