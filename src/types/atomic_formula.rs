//! Contains atomic formulae.

use crate::types::Predicate;
use std::ops::Deref;

/// An atomic formula.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AtomicFormula<'a, T> {
    Equality(EqualityAtomicFormula<T>),
    Predicate(PredicateAtomicFormula<'a, T>),
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct EqualityAtomicFormula<T> {
    first: T,
    second: T,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct PredicateAtomicFormula<'a, T> {
    predicate: Predicate<'a>,
    values: Vec<T>,
}

impl<T> EqualityAtomicFormula<T> {
    pub const fn new(first: T, second: T) -> Self {
        Self { first, second }
    }

    /// Gets a reference to the first element.
    pub const fn first_ref(&self) -> &T {
        &self.first
    }

    /// Gets a reference to the second element.
    pub const fn second_ref(&self) -> &T {
        &self.second
    }
}

impl<'a, T> PredicateAtomicFormula<'a, T> {
    pub const fn new(predicate: Predicate<'a>, values: Vec<T>) -> Self {
        Self { predicate, values }
    }

    /// Gets a reference to the values.
    pub fn as_slice(&self) -> &[T] {
        self.values.as_slice()
    }
}

impl<'a, T> From<EqualityAtomicFormula<T>> for AtomicFormula<'a, T> {
    fn from(value: EqualityAtomicFormula<T>) -> Self {
        AtomicFormula::Equality(value)
    }
}

impl<'a, T> From<PredicateAtomicFormula<'a, T>> for AtomicFormula<'a, T> {
    fn from(value: PredicateAtomicFormula<'a, T>) -> Self {
        AtomicFormula::Predicate(value)
    }
}

impl<T> From<(T, T)> for EqualityAtomicFormula<T> {
    fn from(value: (T, T)) -> Self {
        EqualityAtomicFormula::new(value.0, value.1)
    }
}

impl<'a, T> From<(Predicate<'a>, Vec<T>)> for PredicateAtomicFormula<'a, T> {
    fn from(value: (Predicate<'a>, Vec<T>)) -> Self {
        PredicateAtomicFormula::new(value.0, value.1)
    }
}

impl<'a, T> Deref for PredicateAtomicFormula<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}