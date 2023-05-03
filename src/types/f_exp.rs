//! Contains function expressions via the [`FExp`] type.

use crate::types::{BinaryOp, FHead, MultiOp, Number};

/// A function/fluent expression used e.g. in a [`DurationValue`](crate::types::DurationValue).
///
/// ## Requirements
/// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`FExp`] itself, as well as [`PEffect`](crate::PEffect), [`DurationValue`](crate::DurationValue),
/// [`FExpDa`](crate::FExpDa) and [`FExpT`](crate::FExpT).
#[derive(Debug, Clone, PartialEq)]
pub enum FExp<'a> {
    /// A numerical expression.
    Number(Number),
    /// A function that derives a value.
    Function(FHead<'a>),
    /// The negative value of a function expression.
    Negative(Box<FExp<'a>>),
    /// An operation applied to two function expressions.
    BinaryOp(BinaryOp, Box<FExp<'a>>, Box<FExp<'a>>),
    /// An operation applied to two or more function expressions.
    MultiOp(MultiOp, Box<FExp<'a>>, Vec<FExp<'a>>),
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
