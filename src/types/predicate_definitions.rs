//! Provides predicate definitions;

use crate::types::AtomicFormulaSkeleton;
use std::ops::Deref;

/// A set of predicate definitions.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PredicateDefinitions<'a>(Vec<AtomicFormulaSkeleton<'a>>);

impl<'a> PredicateDefinitions<'a> {
    pub const fn new(predicates: Vec<AtomicFormulaSkeleton<'a>>) -> Self {
        Self(predicates)
    }
}

impl<'a> Deref for PredicateDefinitions<'a> {
    type Target = [AtomicFormulaSkeleton<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> From<Vec<AtomicFormulaSkeleton<'a>>> for PredicateDefinitions<'a> {
    fn from(value: Vec<AtomicFormulaSkeleton<'a>>) -> Self {
        PredicateDefinitions::new(value)
    }
}

impl<'a> FromIterator<AtomicFormulaSkeleton<'a>> for PredicateDefinitions<'a> {
    fn from_iter<T: IntoIterator<Item = AtomicFormulaSkeleton<'a>>>(iter: T) -> Self {
        PredicateDefinitions::new(iter.into_iter().collect())
    }
}
