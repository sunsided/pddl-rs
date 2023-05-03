//! Contains the [`DurativeActionSymbol`] type.

use crate::types::Name;
use std::ops::Deref;

/// A durative action symbol.
///
/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct DurativeActionSymbol<'a>(Name<'a>);

impl<'a> DurativeActionSymbol<'a> {
    #[inline(always)]
    pub const fn new(name: Name<'a>) -> Self {
        Self(name)
    }

    #[inline(always)]
    pub const fn from_str(name: &'a str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name<'a>) -> Self {
        Self(name)
    }
}

impl<'a, T> From<T> for DurativeActionSymbol<'a>
where
    T: Into<Name<'a>>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        DurativeActionSymbol::new(value.into())
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
