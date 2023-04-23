//! Contains function expressions via the [`FExp`] type.

use crate::types::{BinaryOp, FHead, MultiOp, Number};

/// An f-exp.
///
/// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
#[derive(Debug, Clone, PartialEq)]
pub enum FExp<'a> {
    Number(Number),
    BinaryOp(BinaryOp, Box<FExp<'a>>, Box<FExp<'a>>),
    MultiOp(MultiOp, Box<FExp<'a>>, Vec<FExp<'a>>),
    Negative(Box<FExp<'a>>),
    Function(FHead<'a>),
}

impl<'a> FExp<'a> {
    #[inline(always)]
    pub fn new_number<N: Into<Number>>(number: N) -> Self {
        Self::Number(number.into())
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

    pub const fn new_function(f_head: FHead<'a>) -> Self {
        Self::Function(f_head)
    }
}
