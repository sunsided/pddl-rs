//! Provides predicate definitions.

use crate::types::AtomicFormulaSkeleton;
use std::ops::Deref;

/// A set of predicate definitions.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct PredicateDefinitions(Vec<AtomicFormulaSkeleton>);

impl PredicateDefinitions {
    pub fn new<I: IntoIterator<Item = AtomicFormulaSkeleton>>(predicates: I) -> Self {
        Self::from(predicates.into_iter().collect())
    }

    pub const fn from(predicates: Vec<AtomicFormulaSkeleton>) -> Self {
        Self(predicates)
    }

    /// Gets the values.
    pub fn values(&self) -> &[AtomicFormulaSkeleton] {
        self.0.as_slice()
    }
}

impl Deref for PredicateDefinitions {
    type Target = [AtomicFormulaSkeleton];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl From<Vec<AtomicFormulaSkeleton>> for PredicateDefinitions {
    fn from(value: Vec<AtomicFormulaSkeleton>) -> Self {
        PredicateDefinitions::new(value)
    }
}

impl FromIterator<AtomicFormulaSkeleton> for PredicateDefinitions {
    fn from_iter<T: IntoIterator<Item = AtomicFormulaSkeleton>>(iter: T) -> Self {
        PredicateDefinitions::new(iter)
    }
}
