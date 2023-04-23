//! Contains the [`Timeless`] type.

use crate::types::NameLiteral;
use std::ops::Deref;

/// A timeless predicate.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Timeless<'a>(Vec<NameLiteral<'a>>);

impl<'a> Timeless<'a> {
    pub fn new(literal: Vec<NameLiteral<'a>>) -> Self {
        Self(literal)
    }

    /// Gets the literals.
    pub fn values(&self) -> &[NameLiteral<'a>] {
        &self.0.as_slice()
    }
}

impl<'a> FromIterator<NameLiteral<'a>> for Timeless<'a> {
    fn from_iter<T: IntoIterator<Item = NameLiteral<'a>>>(iter: T) -> Self {
        Timeless::new(iter.into_iter().collect())
    }
}

impl<'a> Deref for Timeless<'a> {
    type Target = [NameLiteral<'a>];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
