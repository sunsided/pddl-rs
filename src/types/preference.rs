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
pub struct Preference<'a>(Option<PreferenceName<'a>>, GoalDefinition<'a>); // TODO: A similar type is used for PrefConGD

impl<'a> Preference<'a> {
    pub const fn new(name: Option<PreferenceName<'a>>, gd: GoalDefinition<'a>) -> Self {
        Self(name, gd)
    }

    /// Gets the optional preference name.
    pub fn name(&self) -> &Option<PreferenceName<'a>> {
        &self.0
    }

    /// Gets the goal definition.
    pub fn goal(&self) -> &GoalDefinition<'a> {
        &self.1
    }

    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }
}

impl<'a> From<(Option<PreferenceName<'a>>, GoalDefinition<'a>)> for Preference<'a> {
    fn from(value: (Option<PreferenceName<'a>>, GoalDefinition<'a>)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<'a> From<(PreferenceName<'a>, GoalDefinition<'a>)> for Preference<'a> {
    fn from(value: (PreferenceName<'a>, GoalDefinition<'a>)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}

impl<'a> From<GoalDefinition<'a>> for Preference<'a> {
    fn from(value: GoalDefinition<'a>) -> Self {
        Self::new(None, value)
    }
}
