//! Contains the [`DurativeActionSymbol`] type..

use crate::types::utility::Name;
use std::ops::Deref;

/// A durative action symbol.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct DurativeActionSymbol<'a>(Name<'a>);

impl<'a> DurativeActionSymbol<'a> {
    #[inline(always)]
    pub const fn from_str(name: &'a str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a> From<Name<'a>> for DurativeActionSymbol<'a> {
    #[inline(always)]
    fn from(value: Name<'a>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for DurativeActionSymbol<'a> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Self(Name::new(value))
    }
}

impl<'a> AsRef<Name<'a>> for DurativeActionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for DurativeActionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for DurativeActionSymbol<'a> {
    type Target = Name<'a>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
