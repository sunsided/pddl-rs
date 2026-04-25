//! Contains the [`ProblemGoalDefinition`] type.

use crate::types::pre_gd::PreconditionGoalDefinitions;
use crate::PreconditionGoalDefinition;
use std::ops::Deref;

/// A problem goal definition; wraps a [`PreconditionGoalDefinitions`].
///
/// # BNF
/// Corresponds to the `(:goal ...)` section of a PDDL problem definition.
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[doc(alias("goal"))]
#[derive(Debug, Clone, PartialEq)]
pub struct ProblemGoalDefinition(PreconditionGoalDefinitions);

impl ProblemGoalDefinition {
    pub const fn new(gd: PreconditionGoalDefinitions) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PreconditionGoalDefinitions {
        &self.0
    }
}

impl PartialEq<PreconditionGoalDefinitions> for ProblemGoalDefinition {
    fn eq(&self, other: &PreconditionGoalDefinitions) -> bool {
        self.0.eq(other)
    }
}

impl From<PreconditionGoalDefinitions> for ProblemGoalDefinition {
    fn from(value: PreconditionGoalDefinitions) -> Self {
        Self::new(value)
    }
}

impl From<PreconditionGoalDefinition> for ProblemGoalDefinition {
    fn from(value: PreconditionGoalDefinition) -> Self {
        Self::new(PreconditionGoalDefinitions::from(value))
    }
}

impl FromIterator<PreconditionGoalDefinition> for ProblemGoalDefinition {
    fn from_iter<T: IntoIterator<Item = PreconditionGoalDefinition>>(iter: T) -> Self {
        ProblemGoalDefinition::new(PreconditionGoalDefinitions::from_iter(iter))
    }
}

impl Deref for ProblemGoalDefinition {
    type Target = PreconditionGoalDefinitions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProblemGoalDefinition> for PreconditionGoalDefinitions {
    fn from(val: ProblemGoalDefinition) -> Self {
        val.0
    }
}

/// Alias for [`ProblemGoalDefinition`].
#[deprecated(since = "0.2.0", note = "Use `ProblemGoalDefinition` instead")]
pub type GoalDef = ProblemGoalDefinition;
