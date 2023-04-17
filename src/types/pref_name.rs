//! Provides names for preferences.

use crate::types::Name;
use std::ops::Deref;

/// A name of a preference.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreferenceName<'a>(Name<'a>);

impl<'a> PreferenceName<'a> {
    pub const fn new(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a> Deref for PreferenceName<'a> {
    type Target = Name<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> From<T> for PreferenceName<'a>
where
    T: Into<Name<'a>>,
{
    fn from(value: T) -> Self {
        PreferenceName::new(value.into())
    }
}
