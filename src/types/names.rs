//! Contains names.

use std::ops::Deref;

/// A name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Name<'a>(&'a str);

impl<'a> Name<'a> {
    pub const fn from_str(name: &'a str) -> Self {
        Self(name)
    }
}

impl<'a> From<&'a str> for Name<'a> {
    fn from(value: &'a str) -> Self {
        Self::from_str(value)
    }
}

impl<'a> AsRef<str> for Name<'a> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<'a> Deref for Name<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
