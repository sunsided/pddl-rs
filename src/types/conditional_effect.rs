//! Contains conditional effects via the [`ConditionalEffect`] type.

use crate::types::PEffect;

/// A conditional effect as used by [`CEffect::When`](crate::types::CEffect::When) and [`TimedEffect::Conditional`](crate::types::TimedEffect::Conditional).
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalEffect<'a> {
    /// Exactly the specified effect applies.
    Single(PEffect<'a>),
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<PEffect<'a>>),
}

impl<'a> ConditionalEffect<'a> {
    pub const fn new(effect: PEffect<'a>) -> Self {
        Self::Single(effect)
    }
    pub const fn new_and(effect: Vec<PEffect<'a>>) -> Self {
        Self::All(effect)
    }
}

impl<'a> From<PEffect<'a>> for ConditionalEffect<'a> {
    fn from(value: PEffect<'a>) -> Self {
        ConditionalEffect::new(value)
    }
}

impl<'a> From<Vec<PEffect<'a>>> for ConditionalEffect<'a> {
    fn from(value: Vec<PEffect<'a>>) -> Self {
        ConditionalEffect::new_and(value)
    }
}

impl<'a> FromIterator<PEffect<'a>> for ConditionalEffect<'a> {
    fn from_iter<T: IntoIterator<Item = PEffect<'a>>>(iter: T) -> Self {
        ConditionalEffect::new_and(iter.into_iter().collect())
    }
}
