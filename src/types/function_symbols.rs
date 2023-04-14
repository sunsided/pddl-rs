//! Contains function symbols.

use crate::types::Name;
use std::ops::Deref;

/// A function symbol name.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct FunctionSymbol<'a>(Name<'a>);

impl<'a> FunctionSymbol<'a> {
    #[inline(always)]
    pub const fn from_str(name: &'a str) -> Self {
        Self(Name::from_str(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a> From<Name<'a>> for FunctionSymbol<'a> {
    #[inline(always)]
    fn from(value: Name<'a>) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a str> for FunctionSymbol<'a> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Self(Name::from_str(value))
    }
}

impl<'a> AsRef<Name<'a>> for FunctionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &Name<'a> {
        &self.0
    }
}

impl<'a> AsRef<str> for FunctionSymbol<'a> {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> Deref for FunctionSymbol<'a> {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
