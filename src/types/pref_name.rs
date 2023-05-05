//! Provides names for preferences.

use crate::types::Name;
use std::ops::Deref;

/// A name of a preference.
///
/// ## Usage
/// Used by [`PrefGD`](crate::PreferenceGD), [`PrefTimedGD`](crate::PrefTimedGD),
/// [`PrefConGD`](crate::PrefConGD) and [`MetricFExp`](crate::MetricFExp).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreferenceName(Name);

impl PreferenceName {
    #[inline(always)]
    pub fn new<N: Into<Name>>(name: N) -> Self {
        Self(name.into())
    }

    #[inline(always)]
    pub fn from_str(name: &str) -> Self {
        Self(Name::new(name))
    }

    #[inline(always)]
    pub const fn from_static(name: &'static str) -> Self {
        Self(Name::new_static(name))
    }

    #[inline(always)]
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

impl<'a, T> From<T> for PreferenceName
where
    T: Into<Name>,
{
    fn from(value: T) -> Self {
        PreferenceName::new(value.into())
    }
}
