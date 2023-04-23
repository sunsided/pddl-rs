//! Contains the [`Objects`] type.

use crate::types::{Name, Typed, TypedNames};
use std::ops::Deref;

/// A list of objects.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Objects<'a>(TypedNames<'a>);

impl<'a> Objects<'a> {
    pub fn new<I: IntoIterator<Item = Typed<'a, Name<'a>>>>(objects: I) -> Self {
        Self(objects.into_iter().collect())
    }
}

impl<'a> From<TypedNames<'a>> for Objects<'a> {
    fn from(value: TypedNames<'a>) -> Self {
        Self(value)
    }
}

impl<'a> FromIterator<Typed<'a, Name<'a>>> for Objects<'a> {
    fn from_iter<T: IntoIterator<Item = Typed<'a, Name<'a>>>>(iter: T) -> Self {
        Objects::new(TypedNames::from_iter(iter))
    }
}

impl<'a> Deref for Objects<'a> {
    type Target = TypedNames<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
