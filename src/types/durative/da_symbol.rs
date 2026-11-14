//! Contains the [`DurativeActionSymbol`] type.

use crate::types::Name;
use std::ops::Deref;

/// A durative action symbol.
///
/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct DurativeActionSymbol(Name);

impl DurativeActionSymbol {
    /// Creates a new durative action symbol from a [`Name`].
    #[inline]
    #[must_use]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    /// Creates a new durative action symbol from a string slice.
    #[inline]
    #[doc(alias = "new_string")]
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self(Name::new(name))
    }

    /// Creates a new durative action symbol from a string slice; same as [`string`](Self::string).
    #[inline]
    #[must_use]
    pub fn new_string(name: &str) -> Self {
        Self::string(name)
    }

    /// Creates a new durative action symbol from a [`Name`]; same as [`new`](Self::new).
    #[inline]
    #[must_use]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<T> From<T> for DurativeActionSymbol
where
    T: Into<Name>,
{
    #[inline]
    fn from(value: T) -> Self {
        DurativeActionSymbol::new(value.into())
    }
}

impl AsRef<Name> for DurativeActionSymbol {
    #[inline]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for DurativeActionSymbol {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for DurativeActionSymbol {
    type Target = Name;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
