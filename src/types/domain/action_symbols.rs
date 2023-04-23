//! Contains action symbols.

use crate::types::utility::Name;
use std::ops::Deref;

/// An action symbol name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ActionSymbol<'a>(Name<'a>);

impl<'a> ActionSymbol<'a> {
    #[inline(always)]
    pub const fn from_str(name: &'a str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a> From<Name<'a>> for ActionSymbol<'a> {
    #[inline(always)]
    fn from(value: Name<'a>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for ActionSymbol<'a> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Self(Name::new(value))
    }
}

impl<'a> AsRef<Name<'a>> for ActionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for ActionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for ActionSymbol<'a> {
    type Target = Name<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
