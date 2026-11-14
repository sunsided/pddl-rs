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
    /// Creates a numeric literal.
    #[inline]
    #[doc(alias = "new_number")]
    #[must_use]
    pub fn number<N: Into<Number>>(number: N) -> Self {
        Self::Number(number.into())
    }

    /// Creates a numeric literal.
    #[must_use]
    pub fn new_number<N: Into<Number>>(number: N) -> Self {
        Self::number(number)
    }

    /// Creates a binary operation, `(<binary-op> <f-exp> <f-exp>)`.
    #[doc(alias = "new_binary_op")]
    #[must_use]
    pub fn binary_op(op: BinaryOp, lhs: FluentExpression, rhs: FluentExpression) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    /// Creates a binary operation, `(<binary-op> <f-exp> <f-exp>)`.
    #[must_use]
    pub fn new_binary_op(op: BinaryOp, lhs: FluentExpression, rhs: FluentExpression) -> Self {
        Self::binary_op(op, lhs, rhs)
    }

    /// Creates an operation over two or more operands, `(<multi-op> <f-exp> <f-exp>+)`.
    #[doc(alias = "new_multi_op")]
    #[must_use]
    pub fn multi_op<I: IntoIterator<Item = FluentExpression>>(
        op: MultiOp,
        lhs: FluentExpression,
        rhs: I,
    ) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    /// Creates an operation over two or more operands, `(<multi-op> <f-exp> <f-exp>+)`.
    #[must_use]
    pub fn new_multi_op<I: IntoIterator<Item = FluentExpression>>(
        op: MultiOp,
        lhs: FluentExpression,
        rhs: I,
    ) -> Self {
        Self::multi_op(op, lhs, rhs)
    }

    /// Creates a negation, `(- <f-exp>)`.
    #[doc(alias = "new_negative")]
    #[must_use]
    pub fn negative(value: FluentExpression) -> Self {
        Self::Negative(Box::new(value))
    }

    /// Creates a negation, `(- <f-exp>)`.
    #[must_use]
    pub fn new_negative(value: FluentExpression) -> Self {
        Self::negative(value)
    }

    /// Creates a fluent expression from a function head.
    #[doc(alias = "new_function")]
    #[must_use]
    pub const fn function(f_head: FunctionHead) -> Self {
        Self::Function(f_head)
    }

    /// Creates a fluent expression from a function head.
    #[must_use]
    pub fn new_function(f_head: FunctionHead) -> Self {
        Self::function(f_head)
    }
}

/// Alias for [`FluentExpression`]; matches BNF `<f-exp>`.
#[deprecated(since = "0.2.0", note = "Use `FluentExpression` instead")]
pub type FExp = FluentExpression;
