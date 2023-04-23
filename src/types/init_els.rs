//! Contains the [`InitElements`] type.

use crate::types::InitElement;
use std::ops::Deref;

/// A wrapper around a list of [`InitElement`] values.
#[derive(Debug, Clone, PartialEq)]
pub struct InitElements<'a>(Vec<InitElement<'a>>);

impl<'a> InitElements<'a> {
    pub const fn new(iter: Vec<InitElement<'a>>) -> Self {
        Self(iter)
    }

    /// Gets the values.
    pub fn values(&self) -> &[InitElement<'a>] {
        self.0.as_slice()
    }
}

impl<'a> Deref for InitElements<'a> {
    type Target = [InitElement<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl<'a> FromIterator<InitElement<'a>> for InitElements<'a> {
    fn from_iter<T: IntoIterator<Item = InitElement<'a>>>(iter: T) -> Self {
        InitElements::new(iter.into_iter().collect())
    }
}
