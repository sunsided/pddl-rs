//! Contains durative function expressions via the [`FExpDa`] type.

use crate::types::{AssignOp, BinaryOp, FExp, FHead, MultiOp};

/// ## Usage
/// Used by [`FExpDa`] itself, as well as [`FAssignDa`](crate::FAssignDa).
#[derive(Debug, Clone, PartialEq)]
pub enum FExpDa {
    Assign(AssignOp, FHead, Box<FExpDa>),
    BinaryOp(BinaryOp, Box<FExpDa>, Box<FExpDa>),
    MultiOp(MultiOp, Box<FExpDa>, Vec<FExpDa>),
    Negative(Box<FExpDa>),
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities).
    Duration,
    FExp(FExp),
}

impl FExpDa {
    pub const fn new_duration() -> Self {
        Self::Duration
    }

    pub fn new_binary_op(op: BinaryOp, lhs: FExpDa, rhs: FExpDa) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multi_op<I: IntoIterator<Item = FExpDa>>(op: MultiOp, lhs: FExpDa, rhs: I) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    pub fn new_negative(value: FExpDa) -> Self {
        Self::Negative(Box::new(value))
    }

    pub const fn new_f_exp(f_head: FExp) -> Self {
        Self::FExp(f_head)
    }
}

impl From<FExp> for FExpDa {
    fn from(value: FExp) -> Self {
        FExpDa::new_f_exp(value)
    }
}
