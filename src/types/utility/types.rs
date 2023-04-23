//! Provides type definitions;

use crate::types::utility::{Name, TypedList};
use std::ops::Deref;

/// A set of types.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Types<'a>(TypedList<'a, Name<'a>>);

impl<'a> Types<'a> {
    pub const fn new(predicates: TypedList<'a, Name<'a>>) -> Self {
        Self(predicates)
    }
}

impl<'a> Deref for Types<'a> {
    type Target = TypedList<'a, Name<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<TypedList<'a, Name<'a>>> for Types<'a> {
    fn from(value: TypedList<'a, Name<'a>>) -> Self {
        Types::new(value)
    }
}
