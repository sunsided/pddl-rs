//! Contains the [`DurativeActionSymbol`] type.

use crate::types::Name;
use std::ops::Deref;

/// A durative action symbol.
///
/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct DurativeActionSymbol(Name);

impl DurativeActionSymbol {
    #[inline(always)]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    #[inline(always)]
    pub fn from_str(name: &str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<'a, T> From<T> for DurativeActionSymbol
where
    T: Into<Name>,
{
    #[inline(always)]
    fn from(value: T) -> Self {
        DurativeActionSymbol::new(value.into())
    }
}

impl AsRef<Name> for DurativeActionSymbol {
    #[inline(always)]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for DurativeActionSymbol {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for DurativeActionSymbol {
    type Target = Name;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
