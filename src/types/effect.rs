//! Contains effects via the [`Effect`] type.

use crate::types::iterators::FlatteningIntoIterator;
use crate::types::CEffect;

/// An effect. Occurs e.g. in a [`ActionDefinition`](crate::types::ActionDefinition).
///
/// ## Usage
/// Used by [`ActionDefinition`](crate::ActionDefinition) and [`CEffect`].
#[derive(Debug, Clone, PartialEq)]
pub enum Effect<'a> {
    /// Exactly the specified effect applies.
    Single(CEffect<'a>), // TODO: Unify with `All` variant; this is just a single-element vector and according to spec, this vector may be empty.
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<CEffect<'a>>),
}

impl<'a> IntoIterator for Effect<'a> {
    type Item = CEffect<'a>;
    type IntoIter = FlatteningIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Effect::Single(item) => FlatteningIntoIterator::new(item),
            Effect::All(vec) => FlatteningIntoIterator::new_vec(vec),
        }
    }
}

impl<'a> Effect<'a> {
    pub const fn new(effect: CEffect<'a>) -> Self {
        Self::Single(effect)
    }
    pub fn new_and<I: IntoIterator<Item = CEffect<'a>>>(effect: I) -> Self {
        Self::All(effect.into_iter().collect())
    }
}

impl<'a> From<CEffect<'a>> for Effect<'a> {
    fn from(value: CEffect<'a>) -> Self {
        Effect::new(value)
    }
}

impl<'a> From<Vec<CEffect<'a>>> for Effect<'a> {
    fn from(value: Vec<CEffect<'a>>) -> Self {
        Effect::new_and(value)
    }
}

impl<'a> FromIterator<CEffect<'a>> for Effect<'a> {
    fn from_iter<T: IntoIterator<Item = CEffect<'a>>>(iter: T) -> Self {
        Effect::new_and(iter)
    }
}
