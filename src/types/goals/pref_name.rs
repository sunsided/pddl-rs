//! Provides names for preferences.

use crate::types::Name;
use std::ops::Deref;

/// A name of a preference.
///
/// ## Usage
/// Used by [`Preference`](crate::Preference), [`PreferenceTimedGoalDescription`](crate::PrefTimedGD),
/// [`PreferenceConstraintGoalDescription`](crate::PrefConGD) and [`MetricFluentExpression`](crate::MetricFluentExpression).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreferenceName(Name);

impl PreferenceName {
    /// Creates a preference name from anything convertible into a [`Name`].
    #[inline]
    #[must_use]
    pub fn new<N: Into<Name>>(name: N) -> Self {
        Self(name.into())
    }

    /// Creates a preference name from a string slice.
    #[inline]
    #[doc(alias = "new_string")]
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self(Name::new(name))
    }

    /// Creates a preference name from a string slice. Same as [`string`](Self::string).
    #[inline]
    #[must_use]
    pub fn new_string(name: &str) -> Self {
        Self::string(name)
    }

    /// Creates a preference name from a static string.
    #[inline]
    #[must_use]
    pub const fn from_static(name: &'static str) -> Self {
        Self(Name::new_static(name))
    }

    /// Creates a preference name from a [`Name`].
    #[inline]
    #[must_use]
    pub const fn from_name(name: Name) -> Self {
        Self(name)
    }
}

impl Deref for PreferenceName {
    type Target = Name;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for PreferenceName
where
    T: Into<Name>,
{
    fn from(value: T) -> Self {
        PreferenceName::new(value.into())
    }
}
