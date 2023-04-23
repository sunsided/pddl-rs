//! Contains the [`FAssignDa`] type.

use crate::types::{AssignOp, FExpDa, FHead};

/// An f-assign-da.
#[derive(Debug, Clone, PartialEq)]
pub struct FAssignDa<'a>(AssignOp, FHead<'a>, FExpDa<'a>);

impl<'a> FAssignDa<'a> {
    pub const fn new(comp: AssignOp, head: FHead<'a>, exp: FExpDa<'a>) -> Self {
        Self(comp, head, exp)
    }
}

impl<'a> From<(AssignOp, FHead<'a>, FExpDa<'a>)> for FAssignDa<'a> {
    fn from(value: (AssignOp, FHead<'a>, FExpDa<'a>)) -> Self {
        FAssignDa::new(value.0, value.1, value.2)
    }
}
