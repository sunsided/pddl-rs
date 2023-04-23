//! Contains the f-exp type.

use crate::types::domain::{BinaryOp, FHead, MultiOp};

/// An f-exp.
///
/// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
#[derive(Debug, Clone, PartialEq)]
pub enum FExp<'a> {
    Number(f32),
    BinaryOp(BinaryOp, Box<FExp<'a>>, Box<FExp<'a>>),
    MultiOp(MultiOp, Box<FExp<'a>>, Vec<FExp<'a>>),
    Negative(Box<FExp<'a>>),
    FHead(FHead<'a>),
}

impl<'a> FExp<'a> {
    pub const fn new_number(number: f32) -> Self {
        Self::Number(number)
    }

    pub fn new_binary_op(op: BinaryOp, lhs: FExp<'a>, rhs: FExp<'a>) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multi_op<I: IntoIterator<Item = FExp<'a>>>(
        op: MultiOp,
        lhs: FExp<'a>,
        rhs: I,
    ) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    pub fn new_negative(value: FExp<'a>) -> Self {
        Self::Negative(Box::new(value))
    }

    pub const fn new_f_head(f_head: FHead<'a>) -> Self {
        Self::FHead(f_head)
    }
}
