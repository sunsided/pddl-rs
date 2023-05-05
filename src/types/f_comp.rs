//! Contains function expression comparisons via the [`FComp`] type.

use crate::types::{BinaryComp, FExp};

/// An fluent comparison used as part of a [`GoalDefinition`](crate::GoalDefinition)
/// when [`NumericFluents`](crate::Requirement::NumericFluents) is allowed.
///
/// ## Usage
/// Used by [`GoalDefinition`](crate::GoalDefinition).
#[derive(Debug, Clone, PartialEq)]
pub struct FComp(BinaryComp, FExp, FExp);

impl FComp {
    pub const fn new(comp: BinaryComp, lhs: FExp, rhs: FExp) -> Self {
        Self(comp, lhs, rhs)
    }

    /// Returns the comparison operator.
    pub const fn comparison(&self) -> &BinaryComp {
        &self.0
    }

    /// Returns the first operand.
    pub const fn first(&self) -> &FExp {
        &self.1
    }

    /// Returns the second operand.
    pub const fn second(&self) -> &FExp {
        &self.2
    }
}

impl From<(BinaryComp, FExp, FExp)> for FComp {
    fn from(value: (BinaryComp, FExp, FExp)) -> Self {
        FComp::new(value.0, value.1, value.2)
    }
}
