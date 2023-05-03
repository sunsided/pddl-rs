//! Contains the [`GoalDef`] type.

use crate::types::PreconditionGoalDefinition;
use std::ops::Deref;

/// A problem goal definition; wraps a [`PreconditionGoalDefinition`].
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[derive(Debug, Clone, PartialEq)]
pub struct GoalDef<'a>(PreconditionGoalDefinition<'a>);

impl<'a> GoalDef<'a> {
    pub const fn new(gd: PreconditionGoalDefinition<'a>) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PreconditionGoalDefinition<'a> {
        &self.0
    }
}

impl<'a> PartialEq<PreconditionGoalDefinition<'a>> for GoalDef<'a> {
    fn eq(&self, other: &PreconditionGoalDefinition<'a>) -> bool {
        self.0.eq(other)
    }
}

impl<'a> From<PreconditionGoalDefinition<'a>> for GoalDef<'a> {
    fn from(value: PreconditionGoalDefinition<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> Deref for GoalDef<'a> {
    type Target = PreconditionGoalDefinition<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Into<PreconditionGoalDefinition<'a>> for GoalDef<'a> {
    fn into(self) -> PreconditionGoalDefinition<'a> {
        self.0
    }
}
