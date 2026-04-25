//! Contains the [`PreferenceTimedGoalDefinition`] type.

use crate::types::{PreferenceName, TimedGoalDefinition};

/// A (preferred) timed goal definition.
///
/// ## Usage
/// Used by [`DurativeActionGoalDefinition`](crate::DurativeActionGoalDefinition).
#[doc(alias("pref-timed-GD"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceTimedGoalDefinition {
    Required(TimedGoalDefinition),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName>, TimedGoalDefinition),
}

impl PreferenceTimedGoalDefinition {
    pub const fn new_required(gd: TimedGoalDefinition) -> Self {
        Self::Required(gd)
    }

    pub const fn new_preference(name: Option<PreferenceName>, gd: TimedGoalDefinition) -> Self {
        Self::Preference(name, gd)
    }
}

impl From<TimedGoalDefinition> for PreferenceTimedGoalDefinition {
    fn from(value: TimedGoalDefinition) -> Self {
        PreferenceTimedGoalDefinition::Required(value)
    }
}

impl From<(Option<PreferenceName>, TimedGoalDefinition)> for PreferenceTimedGoalDefinition {
    fn from(value: (Option<PreferenceName>, TimedGoalDefinition)) -> Self {
        PreferenceTimedGoalDefinition::Preference(value.0, value.1)
    }
}

/// Alias for [`PreferenceTimedGoalDefinition`]; matches BNF `<pref-timed-GD>`.
#[deprecated(since = "0.2.0", note = "Use `PreferenceTimedGoalDefinition` instead")]
pub type PrefTimedGD = PreferenceTimedGoalDefinition;
