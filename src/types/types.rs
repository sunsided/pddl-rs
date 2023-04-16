//! Provides type definitions;

use crate::types::{Name, Typed};
use std::ops::Deref;

/// A set of types.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Types<'a>(Vec<Typed<'a, Name<'a>>>);

impl<'a> Types<'a> {
    pub const fn new(predicates: Vec<Typed<'a, Name<'a>>>) -> Self {
        Self(predicates)
    }
}

impl<'a> Deref for Types<'a> {
    type Target = [Typed<'a, Name<'a>>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> From<Vec<Typed<'a, Name<'a>>>> for Types<'a> {
    fn from(value: Vec<Typed<'a, Name<'a>>>) -> Self {
        Types::new(value)
    }
}

impl<'a> FromIterator<Typed<'a, Name<'a>>> for Types<'a> {
    fn from_iter<T: IntoIterator<Item = Typed<'a, Name<'a>>>>(iter: T) -> Self {
        Types::new(iter.into_iter().collect())
    }
}
