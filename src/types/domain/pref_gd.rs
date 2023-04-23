//! Contains preference goal definitions.

use crate::types::domain::{GoalDefinition, Preference};

/// A preferred goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceGD<'a> {
    GoalDefinition(GoalDefinition<'a>),
    Preference(Preference<'a>),
}

impl<'a> PreferenceGD<'a> {
    pub const fn from_gd(gd: GoalDefinition<'a>) -> Self {
        Self::GoalDefinition(gd)
    }

    pub fn from_preference(pref: Preference<'a>) -> Self {
        Self::Preference(pref)
    }
}

impl<'a> From<GoalDefinition<'a>> for PreferenceGD<'a> {
    fn from(value: GoalDefinition<'a>) -> Self {
        PreferenceGD::from_gd(value)
    }
}

impl<'a> From<Preference<'a>> for PreferenceGD<'a> {
    fn from(value: Preference<'a>) -> Self {
        PreferenceGD::from_preference(value)
    }
}
