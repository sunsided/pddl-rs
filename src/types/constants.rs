//! Provides constant definitions via the [`Constants`] type.

use crate::types::{Name, Typed, TypedNames};
use std::ops::Deref;

/// A set of constants.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Constants<'a>(TypedNames<'a>);

impl<'a> Constants<'a> {
    pub const fn new(predicates: TypedNames<'a>) -> Self {
        Self(predicates)
    }
}

impl<'a> Deref for Constants<'a> {
    type Target = TypedNames<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<TypedNames<'a>> for Constants<'a> {
    fn from(value: TypedNames<'a>) -> Self {
        Constants::new(value)
    }
}

impl<'a> FromIterator<Typed<'a, Name<'a>>> for Constants<'a> {
    fn from_iter<T: IntoIterator<Item = Typed<'a, Name<'a>>>>(iter: T) -> Self {
        Constants::new(TypedNames::from_iter(iter))
    }
}
