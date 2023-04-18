//! Provides function requirements.

use crate::types::Requirement;
use std::ops::Deref;

/// A set of requirements.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Requirements(Vec<Requirement>);

impl Requirements {
    pub fn new<I: IntoIterator<Item = Requirement>>(requirements: I) -> Self {
        Self(requirements.into_iter().collect())
    }
}

impl Deref for Requirements {
    type Target = Vec<Requirement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Requirement>> for Requirements {
    fn from(value: Vec<Requirement>) -> Self {
        Requirements::new(value)
    }
}

impl FromIterator<Requirement> for Requirements {
    fn from_iter<T: IntoIterator<Item = Requirement>>(iter: T) -> Self {
        Requirements::new(iter)
    }
}
