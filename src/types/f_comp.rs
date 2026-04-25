//! Contains function expression comparisons via the [`FluentComparison`] type.

use crate::types::{BinaryComparison, FluentExpression};

/// An fluent comparison used as part of a [`GoalDefinition`](crate::GoalDefinition)
/// when [`NumericFluents`](crate::Requirement::NumericFluents) is allowed.
///
/// ## Usage
/// Used by [`GoalDefinition`](crate::GoalDefinition).
#[doc(alias("f-comp"))]
#[derive(Debug, Clone, PartialEq)]
pub struct FluentComparison(BinaryComparison, FluentExpression, FluentExpression);

impl FluentComparison {
    pub const fn new(comp: BinaryComparison, lhs: FluentExpression, rhs: FluentExpression) -> Self {
        Self(comp, lhs, rhs)
    }

    /// Returns the comparison operator.
    pub const fn comparison(&self) -> &BinaryComparison {
        &self.0
    }

    /// Returns the first operand.
    pub const fn first(&self) -> &FluentExpression {
        &self.1
    }

    /// Returns the second operand.
    pub const fn second(&self) -> &FluentExpression {
        &self.2
    }
}

impl From<(BinaryComparison, FluentExpression, FluentExpression)> for FluentComparison {
    fn from(value: (BinaryComparison, FluentExpression, FluentExpression)) -> Self {
        FluentComparison::new(value.0, value.1, value.2)
    }
}

/// Alias for [`FluentComparison`]; matches BNF `<f-comp>`.
#[deprecated(since = "0.2.0", note = "Use `FluentComparison` instead")]
pub type FComp = FluentComparison;
