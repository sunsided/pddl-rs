//! Contains the [`GoalDef`] type.

use crate::types::pre_gd::PreconditionGoalDefinitions;
use crate::PreconditionGoalDefinition;
use std::ops::Deref;

/// A problem goal definition; wraps a [`PreconditionGoalDefinitions`].
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[derive(Debug, Clone, PartialEq)]
pub struct GoalDef(PreconditionGoalDefinitions);

impl GoalDef {
    pub const fn new(gd: PreconditionGoalDefinitions) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PreconditionGoalDefinitions {
        &self.0
    }
}

impl PartialEq<PreconditionGoalDefinitions> for GoalDef {
    fn eq(&self, other: &PreconditionGoalDefinitions) -> bool {
        self.0.eq(other)
    }
}

impl From<PreconditionGoalDefinitions> for GoalDef {
    fn from(value: PreconditionGoalDefinitions) -> Self {
        Self::new(value)
    }
}

impl From<PreconditionGoalDefinition> for GoalDef {
    fn from(value: PreconditionGoalDefinition) -> Self {
        Self::new(PreconditionGoalDefinitions::from(value))
    }
}

impl FromIterator<PreconditionGoalDefinition> for GoalDef {
    fn from_iter<T: IntoIterator<Item = PreconditionGoalDefinition>>(iter: T) -> Self {
        GoalDef::new(PreconditionGoalDefinitions::from_iter(iter))
    }
}

impl Deref for GoalDef {
    type Target = PreconditionGoalDefinitions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<PreconditionGoalDefinitions> for GoalDef {
    fn into(self) -> PreconditionGoalDefinitions {
        self.0
    }
}
