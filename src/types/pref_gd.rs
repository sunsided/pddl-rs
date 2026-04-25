//! Contains preference goal definitions.

use crate::types::{GoalDefinition, Preference};

/// A preferred goal definition.
///
/// ## Usage
/// Used by [`PreconditionGoalDefinition`](crate::PreconditionGoalDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceGoalDefinition {
    Goal(GoalDefinition),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Preference),
}

impl PreferenceGoalDefinition {
    pub const fn from_gd(gd: GoalDefinition) -> Self {
        Self::Goal(gd)
    }

    pub fn from_preference(pref: Preference) -> Self {
        Self::Preference(pref)
    }
}

impl From<GoalDefinition> for PreferenceGoalDefinition {
    fn from(value: GoalDefinition) -> Self {
        PreferenceGoalDefinition::from_gd(value)
    }
}

impl From<Preference> for PreferenceGoalDefinition {
    fn from(value: Preference) -> Self {
        PreferenceGoalDefinition::from_preference(value)
    }
}

/// Alias for [`PreferenceGoalDefinition`]; matches BNF `<pref-GD>`.
#[deprecated(since = "0.2.0", note = "Use `PreferenceGoalDefinition` instead")]
pub type PreferenceGD = PreferenceGoalDefinition;
