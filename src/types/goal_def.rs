//! Contains the [`GoalDef`] type.

use crate::types::pre_gd::PreconditionGoalDefinitions;
use crate::PreconditionGoalDefinition;
use std::ops::Deref;

/// A problem goal definition; wraps a [`PreconditionGoalDefinition`].
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[derive(Debug, Clone, PartialEq)]
pub struct GoalDef<'a>(PreconditionGoalDefinitions<'a>);

impl<'a> GoalDef<'a> {
    pub const fn new(gd: PreconditionGoalDefinitions<'a>) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PreconditionGoalDefinitions<'a> {
        &self.0
    }
}

impl<'a> PartialEq<PreconditionGoalDefinitions<'a>> for GoalDef<'a> {
    fn eq(&self, other: &PreconditionGoalDefinitions<'a>) -> bool {
        self.0.eq(other)
    }
}

impl<'a> From<PreconditionGoalDefinitions<'a>> for GoalDef<'a> {
    fn from(value: PreconditionGoalDefinitions<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> From<PreconditionGoalDefinition<'a>> for GoalDef<'a> {
    fn from(value: PreconditionGoalDefinition<'a>) -> Self {
        Self::new(PreconditionGoalDefinitions::from(value))
    }
}

impl<'a> Deref for GoalDef<'a> {
    type Target = PreconditionGoalDefinitions<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Into<PreconditionGoalDefinitions<'a>> for GoalDef<'a> {
    fn into(self) -> PreconditionGoalDefinitions<'a> {
        self.0
    }
}
