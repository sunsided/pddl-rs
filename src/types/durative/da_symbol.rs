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
    /// Creates a new durative action symbol from a [`Name`].
    #[inline(always)]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    /// Creates a new durative action symbol from a string slice.
    #[inline(always)]
    #[doc(alias = "new_string")]
    pub fn string(name: &str) -> Self {
        Self(Name::new(name))
    }

    /// Creates a new durative action symbol from a string slice; same as [`string`](Self::string).
    #[inline(always)]
    pub fn new_string(name: &str) -> Self {
        Self::string(name)
    }

    /// Creates a new durative action symbol from a [`Name`]; same as [`new`](Self::new).
    #[inline(always)]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<T> From<T> for DurativeActionSymbol
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
