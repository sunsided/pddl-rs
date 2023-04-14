//! Contains variables.

use crate::types::Name;
use std::ops::Deref;

/// A variable name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct Variable<'a>(Name<'a>);

impl<'a> Variable<'a> {
    #[inline(always)]
    pub const fn from_str(name: &'a str) -> Self {
        Self(Name::from_str(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a> From<Name<'a>> for Variable<'a> {
    #[inline(always)]
    fn from(value: Name<'a>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for Variable<'a> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Self(Name::from_str(value))
    }
}

impl<'a> AsRef<Name<'a>> for Variable<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for Variable<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for Variable<'a> {
    type Target = Name<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
