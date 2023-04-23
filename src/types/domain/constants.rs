//! Provides constant definitions;

use crate::types::domain::{Name, TypedList};
use std::ops::Deref;

/// A set of constants.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Constants<'a>(TypedList<'a, Name<'a>>);

impl<'a> Constants<'a> {
    pub const fn new(predicates: TypedList<'a, Name<'a>>) -> Self {
        Self(predicates)
    }
}

impl<'a> Deref for Constants<'a> {
    type Target = TypedList<'a, Name<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<TypedList<'a, Name<'a>>> for Constants<'a> {
    fn from(value: TypedList<'a, Name<'a>>) -> Self {
        Constants::new(value)
    }
}
