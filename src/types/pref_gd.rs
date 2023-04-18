//! Contains preference goal definitions.

use crate::types::{Preference, GD};

/// A preferred goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceGD<'a> {
    GoalDefinition(GD<'a>),
    Preference(Preference<'a>),
}

impl<'a> PreferenceGD<'a> {
    pub const fn from_gd(gd: GD<'a>) -> Self {
        Self::GoalDefinition(gd)
    }

    pub fn from_preference(pref: Preference<'a>) -> Self {
        Self::Preference(pref)
    }
}

impl<'a> From<GD<'a>> for PreferenceGD<'a> {
    fn from(value: GD<'a>) -> Self {
        PreferenceGD::from_gd(value)
    }
}

impl<'a> From<Preference<'a>> for PreferenceGD<'a> {
    fn from(value: Preference<'a>) -> Self {
        PreferenceGD::from_preference(value)
    }
}
