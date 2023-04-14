//! Contains variables.

use crate::types::Name;
use std::ops::Deref;

/// A variable name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Variable<'a>(Name<'a>);

impl<'a> From<Name<'a>> for Variable<'a> {
    fn from(value: Name<'a>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for Variable<'a> {
    fn from(value: &'a str) -> Self {
        Self(Name::from_str(value))
    }
}

impl<'a> AsRef<Name<'a>> for Variable<'a> {
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for Variable<'a> {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for Variable<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
