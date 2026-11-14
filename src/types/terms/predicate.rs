//! Contains predicates via the [`Predicate`] type.

use crate::types::Name;
use std::ops::Deref;

/// A predicate name.
///
/// ## Usage
/// Used by [`AtomicFormulaSkeleton`](crate::AtomicFormulaSkeleton) and [`AtomicFormula`](crate::AtomicFormula).
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Predicate(Name);

impl Predicate {
    /// Creates a predicate from a [`Name`].
    #[inline]
    #[must_use]
    pub const fn new(name: Name) -> Self {
        Self(name)
    }

    /// Creates a predicate from a string slice.
    #[inline]
    #[doc(alias = "new_string")]
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self(Name::new(name))
    }

    /// Alias for [`Self::string`].
    #[inline]
    #[must_use]
    pub fn new_string(name: &str) -> Self {
        Self::string(name)
    }

    /// Creates a predicate from a `'static` string; usable in `const` contexts.
    #[inline]
    #[must_use]
    pub const fn from_static(name: &'static str) -> Self {
        Self(Name::new_static(name))
    }

    /// Creates a predicate from a [`Name`]; same as [`Self::new`].
    #[inline]
    #[must_use]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl<T> From<T> for Predicate
where
    T: Into<Name>,
{
    #[inline]
    fn from(value: T) -> Self {
        Predicate::new(value.into())
    }
}

impl AsRef<Name> for Predicate {
    #[inline]
    fn as_ref(&self) -> &Name {
        &self.0
    }
}

impl AsRef<str> for Predicate {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for Predicate {
    type Target = Name;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
