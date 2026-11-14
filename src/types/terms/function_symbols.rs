//! Contains function symbols via the [`FunctionSymbol`] type..

use crate::types::Name;
use std::ops::Deref;

/// A function symbol name.
///
/// ## Usage
/// Used by [`FunctionTerm`](crate::FunctionTerm), [`FHead`](crate::FHead),
/// [`BasicFunctionTerm`](crate::BasicFunctionTerm) and [`MetricFluentExpression`](crate::MetricFluentExpression).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FunctionSymbol(Name);

impl FunctionSymbol {
    /// Creates a new function symbol from a name.
    #[inline]
    #[must_use]
    pub fn new(name: Name) -> Self {
        Self(name)
    }

    /// Creates a new function symbol from a string.
    #[inline]
    #[doc(alias = "new_string")]
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self(Name::new(name))
    }

    /// Creates a new function symbol from a string.
    #[inline]
    #[must_use]
    pub fn new_string(name: &str) -> Self {
        Self::string(name)
    }

    /// Creates a new function symbol from a name.
    #[inline]
    #[must_use]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<T> From<T> for FunctionSymbol
where
    T: Into<Name>,
{
    #[inline]
    fn from(value: T) -> Self {
        FunctionSymbol::new(value.into())
    }
}

impl AsRef<Name> for FunctionSymbol {
    #[inline]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for FunctionSymbol {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for FunctionSymbol {
    type Target = Name;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
