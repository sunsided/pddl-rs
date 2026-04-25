//! Contains the [`ProblemConstraintsDef`] type.

use crate::PreferenceConstraintGoalDefinitions;
use std::ops::Deref;

/// A problem constraints definition; wraps a [`PreferenceConstraintGoalDefinitions`].
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProblemConstraintsDef(PreferenceConstraintGoalDefinitions);

impl ProblemConstraintsDef {
    pub const fn new(gd: PreferenceConstraintGoalDefinitions) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PreferenceConstraintGoalDefinitions {
        &self.0
    }
}

impl PartialEq<PreferenceConstraintGoalDefinitions> for ProblemConstraintsDef {
    fn eq(&self, other: &PreferenceConstraintGoalDefinitions) -> bool {
        self.0.eq(other)
    }
}

impl From<PreferenceConstraintGoalDefinitions> for ProblemConstraintsDef {
    fn from(value: PreferenceConstraintGoalDefinitions) -> Self {
        Self::new(value)
    }
}

impl Deref for ProblemConstraintsDef {
    type Target = PreferenceConstraintGoalDefinitions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProblemConstraintsDef> for PreferenceConstraintGoalDefinitions {
    fn from(val: ProblemConstraintsDef) -> Self {
        val.0
    }
}
