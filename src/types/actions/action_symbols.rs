//! Contains action symbols via the [`ActionSymbol`] type.

use crate::types::Name;
use std::ops::Deref;

/// An action symbol name.
///
/// ## Usage
/// Used by [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct ActionSymbol(Name);

impl ActionSymbol {
    /// Creates a new action symbol from a [`Name`].
    #[inline]
    #[must_use]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    /// Creates a new action symbol from a string slice.
    #[inline]
    #[doc(alias = "new_string")]
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self(Name::new(name))
    }

    /// Creates a new action symbol from a string slice; same as [`string`](Self::string).
    #[inline]
    #[must_use]
    pub fn new_string(name: &str) -> Self {
        Self::string(name)
    }

    /// Creates a new action symbol from a [`Name`]; same as [`new`](Self::new).
    #[inline]
    #[must_use]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<T> From<T> for ActionSymbol
where
    T: Into<Name>,
{
    #[inline]
    fn from(value: T) -> Self {
        ActionSymbol::new(value.into())
    }
}

impl AsRef<Name> for ActionSymbol {
    #[inline]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for ActionSymbol {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for ActionSymbol {
    type Target = Name;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
