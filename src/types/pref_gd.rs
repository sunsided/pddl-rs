//! Contains preference goal definitions.

use crate::types::{GoalDefinition, Preference};

/// A preferred goal definition.
///
/// ## Usage
/// Used by [`PreconditionGoalDefinition`](crate::PreconditionGoalDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceGD {
    Goal(GoalDefinition),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Preference),
}

impl PreferenceGD {
    pub const fn from_gd(gd: GoalDefinition) -> Self {
        Self::Goal(gd)
    }

    pub fn from_preference(pref: Preference) -> Self {
        Self::Preference(pref)
    }
}

impl From<GoalDefinition> for PreferenceGD {
    fn from(value: GoalDefinition) -> Self {
        PreferenceGD::from_gd(value)
    }
}

impl From<Preference> for PreferenceGD {
    fn from(value: Preference) -> Self {
        PreferenceGD::from_preference(value)
    }
}
