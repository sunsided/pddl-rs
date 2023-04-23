//! Contains durative function expressions via the [`FExpDa`] type.

use crate::types::{AssignOp, BinaryOp, FExp, FHead, MultiOp};

#[derive(Debug, Clone, PartialEq)]
pub enum FExpDa<'a> {
    Assign(AssignOp, FHead<'a>, Box<FExpDa<'a>>),
    BinaryOp(BinaryOp, Box<FExpDa<'a>>, Box<FExpDa<'a>>),
    MultiOp(MultiOp, Box<FExpDa<'a>>, Vec<FExpDa<'a>>),
    Negative(Box<FExpDa<'a>>),
    /// Requires [DurationInequalities](crate::types::Requirement::DurationInequalities).
    Duration,
    FExp(FExp<'a>),
}

impl<'a> FExpDa<'a> {
    pub const fn new_duration() -> Self {
        Self::Duration
    }

    pub fn new_binary_op(op: BinaryOp, lhs: FExpDa<'a>, rhs: FExpDa<'a>) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multi_op<I: IntoIterator<Item = FExpDa<'a>>>(
        op: MultiOp,
        lhs: FExpDa<'a>,
        rhs: I,
    ) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    pub fn new_negative(value: FExpDa<'a>) -> Self {
        Self::Negative(Box::new(value))
    }

    pub const fn new_f_exp(f_head: FExp<'a>) -> Self {
        Self::FExp(f_head)
    }
}

impl<'a> From<FExp<'a>> for FExpDa<'a> {
    fn from(value: FExp<'a>) -> Self {
        FExpDa::new_f_exp(value)
    }
}
