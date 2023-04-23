//! Provides constant definitions;

use crate::types::TypedNames;
use std::ops::Deref;

/// A set of constants.
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
