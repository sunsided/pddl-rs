//! Contains the durative action assignment expressions via the [`FAssignDa`] type.

use crate::types::{AssignOp, FExpDa, FHead};

/// An f-assign-da.
#[derive(Debug, Clone, PartialEq)]
pub struct FAssignDa<'a>(AssignOp, FHead<'a>, FExpDa<'a>);

impl<'a> FAssignDa<'a> {
    pub const fn new(comp: AssignOp, head: FHead<'a>, exp: FExpDa<'a>) -> Self {
        Self(comp, head, exp)
    }

    /// Returns the operation.
    pub const fn operation(&self) -> &AssignOp {
        &self.0
    }

    /// Returns the function head.
    pub const fn function(&self) -> &FHead<'a> {
        &self.1
    }

    /// Returns the function expression of the durative action.
    pub const fn function_expr(&self) -> &FExpDa<'a> {
        &self.2
    }
}

impl<'a> From<(AssignOp, FHead<'a>, FExpDa<'a>)> for FAssignDa<'a> {
    fn from(value: (AssignOp, FHead<'a>, FExpDa<'a>)) -> Self {
        FAssignDa::new(value.0, value.1, value.2)
    }
}
