//! Contains the durative action assignment expressions via the [`DurativeActionFunctionAssignment`] type.

use crate::types::{AssignOp, DurativeActionFluentExpression, FunctionHead};

/// An timed effect assignment operation. Will perform the
/// specified assignment `at` [`TimeSpecifier`](crate::TimeSpecifier) when
/// [`NumericFluents`](crate::Requirement::NumericFluents) is allowed.
///
/// ## Requirements
/// Requires [`NumericFluents`](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`TimedEffect`](crate::TimedEffect).
#[doc(alias("f-assign-da"))]
#[derive(Debug, Clone, PartialEq)]
pub struct DurativeActionFunctionAssignment(AssignOp, FunctionHead, DurativeActionFluentExpression);

impl DurativeActionFunctionAssignment {
    pub const fn new(
        comp: AssignOp,
        head: FunctionHead,
        exp: DurativeActionFluentExpression,
    ) -> Self {
        Self(comp, head, exp)
    }

    /// Returns the operation.
    pub const fn operation(&self) -> &AssignOp {
        &self.0
    }

    /// Returns the function head.
    pub const fn function(&self) -> &FunctionHead {
        &self.1
    }

    /// Returns the function expression of the durative action.
    pub const fn function_expr(&self) -> &DurativeActionFluentExpression {
        &self.2
    }
}

impl From<(AssignOp, FunctionHead, DurativeActionFluentExpression)>
    for DurativeActionFunctionAssignment
{
    fn from(value: (AssignOp, FunctionHead, DurativeActionFluentExpression)) -> Self {
        DurativeActionFunctionAssignment::new(value.0, value.1, value.2)
    }
}

/// Alias for [`DurativeActionFunctionAssignment`]; matches BNF `<f-assign-da>`.
#[deprecated(
    since = "0.2.0",
    note = "Use `DurativeActionFunctionAssignment` instead"
)]
pub type FAssignDa = DurativeActionFunctionAssignment;
