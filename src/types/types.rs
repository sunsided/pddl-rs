//! Provides type definitions;

use crate::types::TypedNames;
use std::ops::Deref;

/// A set of types.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Types<'a>(TypedNames<'a>);

impl<'a> Types<'a> {
    pub const fn new(predicates: TypedNames<'a>) -> Self {
        Self(predicates)
    }
}

impl<'a> Deref for Types<'a> {
    type Target = TypedNames<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<TypedNames<'a>> for Types<'a> {
    fn from(value: TypedNames<'a>) -> Self {
        Types::new(value)
    }
}
