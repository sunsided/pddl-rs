//! Provides constant definitions;

use crate::types::{Name, Typed};
use std::ops::Deref;

/// A set of constants.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Constants<'a>(Vec<Typed<'a, Name<'a>>>);

impl<'a> Constants<'a> {
    pub const fn new(predicates: Vec<Typed<'a, Name<'a>>>) -> Self {
        Self(predicates)
    }
}

impl<'a> Deref for Constants<'a> {
    type Target = [Typed<'a, Name<'a>>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> From<Vec<Typed<'a, Name<'a>>>> for Constants<'a> {
    fn from(value: Vec<Typed<'a, Name<'a>>>) -> Self {
        Constants::new(value)
    }
}

impl<'a> FromIterator<Typed<'a, Name<'a>>> for Constants<'a> {
    fn from_iter<T: IntoIterator<Item = Typed<'a, Name<'a>>>>(iter: T) -> Self {
        Constants::new(iter.into_iter().collect())
    }
}
