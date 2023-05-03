//! Provides names for preferences.

use crate::types::Name;
use std::ops::Deref;

/// A name of a preference.
///
/// ## Usage
/// Used by [`PrefGD`](crate::PreferenceGD), [`PrefTimedGD`](crate::PrefTimedGD),
/// [`PrefConGD`](crate::PrefConGD) and [`MetricFExp`](crate::MetricFExp).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PreferenceName<'a>(Name<'a>);

impl<'a> PreferenceName<'a> {
    #[inline(always)]
    pub fn new<N: Into<Name<'a>>>(name: N) -> Self {
        Self(name.into())
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

impl<'a> Deref for PreferenceName<'a> {
    type Target = Name<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, T> From<T> for PreferenceName<'a>
where
    T: Into<Name<'a>>,
{
    fn from(value: T) -> Self {
        PreferenceName::new(value.into())
    }
}
