//! Contains the durative action assignment expressions via the [`FAssignDa`] type.

use crate::types::{AssignOp, FExpDa, FHead};

/// An timed effect assignment operation. Will perform the
/// specified assignment `at` [`TimeSpecifier`](crate::TimeSpecifier) when
/// [`NumericFluents`](crate::Requirement::NumericFluents) is allowed.
///
/// ## Requirements
/// Requires [`NumericFluents`](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`TimedEffect`](crate::TimedEffect).
#[derive(Debug, Clone, PartialEq)]
pub struct FAssignDa(AssignOp, FHead, FExpDa);

impl FAssignDa {
    pub const fn new(comp: AssignOp, head: FHead, exp: FExpDa) -> Self {
        Self(comp, head, exp)
    }

    /// Returns the operation.
    pub const fn operation(&self) -> &AssignOp {
        &self.0
    }

    /// Returns the function head.
    pub const fn function(&self) -> &FHead {
        &self.1
    }

    /// Returns the function expression of the durative action.
    pub const fn function_expr(&self) -> &FExpDa {
        &self.2
    }
}

impl From<(AssignOp, FHead, FExpDa)> for FAssignDa {
    fn from(value: (AssignOp, FHead, FExpDa)) -> Self {
        FAssignDa::new(value.0, value.1, value.2)
    }
}
