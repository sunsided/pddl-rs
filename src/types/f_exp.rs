//! Contains function expressions via the [`FluentExpression`] type.

use crate::types::{BinaryOp, FunctionHead, MultiOp, Number};

/// A function/fluent expression used e.g. in a [`DurationValue`](crate::types::DurationValue).
///
/// ## Requirements
/// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`FluentExpression`] itself, as well as [`PrimitiveEffect`](crate::PrimitiveEffect), [`DurationValue`](crate::DurationValue),
/// [`DurativeActionFluentExpression`](crate::DurativeActionFluentExpression) and [`TimedFluentExpression`](crate::TimedFluentExpression).
#[doc(alias("f-exp"))]
#[derive(Debug, Clone, PartialEq)]
pub enum FluentExpression {
    /// A numerical expression.
    Number(Number),
    /// A function that derives a value.
    Function(FunctionHead),
    /// The negative value of a fluent expression.
    Negative(Box<FluentExpression>),
    /// An operation applied to two fluent expressions.
    BinaryOp(BinaryOp, Box<FluentExpression>, Box<FluentExpression>),
    /// An operation applied to two or more fluent expressions.
    MultiOp(MultiOp, Box<FluentExpression>, Vec<FluentExpression>),
}

impl FluentExpression {
    #[inline(always)]
    pub fn new_number<N: Into<Number>>(number: N) -> Self {
        Self::Number(number.into())
    }

    pub fn new_binary_op(op: BinaryOp, lhs: FluentExpression, rhs: FluentExpression) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multi_op<I: IntoIterator<Item = FluentExpression>>(op: MultiOp, lhs: FluentExpression, rhs: I) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    pub fn new_negative(value: FluentExpression) -> Self {
        Self::Negative(Box::new(value))
    }

    pub const fn new_function(f_head: FunctionHead) -> Self {
        Self::Function(f_head)
    }
}

/// Alias for [`FluentExpression`]; matches BNF `<f-exp>`.
#[deprecated(since = "0.2.0", note = "Use `FluentExpression` instead")]
pub type FExp = FluentExpression;
