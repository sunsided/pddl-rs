//! Contains the [`InitElements`] type.

use crate::types::InitElement;
use std::ops::Deref;

/// A wrapper around a list of [`InitElement`] values.
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[derive(Debug, Clone, PartialEq)]
pub struct InitElements(Vec<InitElement>);

impl InitElements {
    pub const fn new(iter: Vec<InitElement>) -> Self {
        Self(iter)
    }

    /// Gets the values.
    pub fn values(&self) -> &[InitElement] {
        self.0.as_slice()
    }
}

impl Deref for InitElements {
    type Target = [InitElement];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl FromIterator<InitElement> for InitElements {
    fn from_iter<T: IntoIterator<Item = InitElement>>(iter: T) -> Self {
        InitElements::new(iter.into_iter().collect())
    }
}
