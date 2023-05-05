//! Contains function expressions via the [`FExp`] type.

use crate::types::{BinaryOp, FHead, MultiOp, Number};

/// A function/fluent expression used e.g. in a [`DurationValue`](crate::types::DurationValue).
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`FExp`] itself, as well as [`PEffect`](crate::PEffect), [`DurationValue`](crate::DurationValue),
/// [`FExpDa`](crate::FExpDa) and [`FExpT`](crate::FExpT).
#[derive(Debug, Clone, PartialEq)]
pub enum FExp {
    /// A numerical expression.
    Number(Number),
    /// A function that derives a value.
    Function(FHead),
    /// The negative value of a function expression.
    Negative(Box<FExp>),
    /// An operation applied to two function expressions.
    BinaryOp(BinaryOp, Box<FExp>, Box<FExp>),
    /// An operation applied to two or more function expressions.
    MultiOp(MultiOp, Box<FExp>, Vec<FExp>),
}

impl FExp {
    #[inline(always)]
    pub fn new_number<N: Into<Number>>(number: N) -> Self {
        Self::Number(number.into())
    }

    pub fn new_binary_op(op: BinaryOp, lhs: FExp, rhs: FExp) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multi_op<I: IntoIterator<Item = FExp>>(op: MultiOp, lhs: FExp, rhs: I) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    pub fn new_negative(value: FExp) -> Self {
        Self::Negative(Box::new(value))
    }

    pub const fn new_function(f_head: FHead) -> Self {
        Self::Function(f_head)
    }
}
