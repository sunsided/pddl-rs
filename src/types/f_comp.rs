//! Contains function expression comparisons via the [`FComp`] type.

use crate::types::{BinaryComp, FExp};

/// An fluent comparison used as part of a [`GoalDefinition`](crate::GoalDefinition)
/// when [`NumericFluents`](crate::Requirement::NumericFluents) is allowed.
///
/// ## Usage
/// Used by [`GoalDefinition`](crate::GoalDefinition).
#[derive(Debug, Clone, PartialEq)]
pub struct FComp<'a>(BinaryComp, FExp<'a>, FExp<'a>);

impl<'a> FComp<'a> {
    pub const fn new(comp: BinaryComp, lhs: FExp<'a>, rhs: FExp<'a>) -> Self {
        Self(comp, lhs, rhs)
    }

    /// Returns the comparison operator.
    pub const fn comparison(&self) -> &BinaryComp {
        &self.0
    }

    /// Returns the first operand.
    pub const fn first(&self) -> &FExp<'a> {
        &self.1
    }

    /// Returns the second operand.
    pub const fn second(&self) -> &FExp<'a> {
        &self.2
    }
}

impl<'a> From<(BinaryComp, FExp<'a>, FExp<'a>)> for FComp<'a> {
    fn from(value: (BinaryComp, FExp<'a>, FExp<'a>)) -> Self {
        FComp::new(value.0, value.1, value.2)
    }
}
