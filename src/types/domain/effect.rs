//! Contains effects.

use crate::types::domain::CEffect;

/// An effect.
#[derive(Debug, Clone, PartialEq)]
pub enum Effect<'a> {
    /// Exactly the specified effect applies.
    Single(CEffect<'a>),
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<CEffect<'a>>),
}

impl<'a> Effect<'a> {
    pub const fn new(effect: CEffect<'a>) -> Self {
        Self::Single(effect)
    }
    pub const fn new_and(effect: Vec<CEffect<'a>>) -> Self {
        Self::All(effect)
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
        Effect::new_and(iter.into_iter().collect())
    }
}
