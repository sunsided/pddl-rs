//! Contains preferences.

use crate::types::{GoalDefinition, PreferenceName};

/// A preference.
///
/// ## Requirements
/// Requires [Preferences](crate::Requirement::Preferences).
///
/// ## Usage
/// Used by [`PreferenceGD`](crate::PreferenceGD).
#[derive(Debug, Clone, PartialEq)]
pub struct Preference(Option<PreferenceName>, GoalDefinition); // TODO: A similar type is used for PrefConGD

impl Preference {
    pub const fn new(name: Option<PreferenceName>, gd: GoalDefinition) -> Self {
        Self(name, gd)
    }

    /// Gets the optional preference name.
    pub fn name(&self) -> &Option<PreferenceName> {
        &self.0
    }

    /// Gets the goal definition.
    pub fn goal(&self) -> &GoalDefinition {
        &self.1
    }

    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }
}

impl From<(Option<PreferenceName>, GoalDefinition)> for Preference {
    fn from(value: (Option<PreferenceName>, GoalDefinition)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(PreferenceName, GoalDefinition)> for Preference {
    fn from(value: (PreferenceName, GoalDefinition)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}

impl From<GoalDefinition> for Preference {
    fn from(value: GoalDefinition) -> Self {
        Self::new(None, value)
    }
}
