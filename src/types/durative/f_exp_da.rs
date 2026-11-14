//! Contains durative function expressions via the [`DurativeActionFluentExpression`] type.

use crate::types::{AssignOp, BinaryOp, FluentExpression, FunctionHead, MultiOp};

/// ## Usage
/// Used by [`DurativeActionFluentExpression`] itself, as well as [`DurativeActionFunctionAssignment`](crate::DurativeActionFunctionAssignment).
#[doc(alias("f-exp-da"))]
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DurativeActionFluentExpression {
    /// An assignment, `(<assign-op> <f-head> <f-exp-da>)`. Not part of the PDDL 3.1
    /// `<f-exp-da>` grammar; the parser never produces it.
    Assign(AssignOp, FunctionHead, Box<DurativeActionFluentExpression>),
    /// A binary operation, `(<binary-op> <f-exp-da> <f-exp-da>)`.
    BinaryOp(
        BinaryOp,
        Box<DurativeActionFluentExpression>,
        Box<DurativeActionFluentExpression>,
    ),
    /// An operation over two or more operands, `(<multi-op> <f-exp-da> <f-exp-da>+)`.
    MultiOp(
        MultiOp,
        Box<DurativeActionFluentExpression>,
        Vec<DurativeActionFluentExpression>,
    ),
    /// A negation, `(- <f-exp-da>)`.
    Negative(Box<DurativeActionFluentExpression>),
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities).
    Duration,
    /// A regular fluent expression, `<f-exp>`.
    FluentExpression(FluentExpression),
}

impl DurativeActionFluentExpression {
    /// Creates the `?duration` expression.
    #[doc(alias = "new_duration")]
    #[must_use]
    pub const fn duration() -> Self {
        Self::Duration
    }

    /// Creates the `?duration` expression.
    #[must_use]
    pub fn new_duration() -> Self {
        Self::duration()
    }

    /// Creates a binary operation, `(<binary-op> <f-exp-da> <f-exp-da>)`.
    #[doc(alias = "new_binary_op")]
    #[must_use]
    pub fn binary_op(
        op: BinaryOp,
        lhs: DurativeActionFluentExpression,
        rhs: DurativeActionFluentExpression,
    ) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    /// Creates a binary operation, `(<binary-op> <f-exp-da> <f-exp-da>)`.
    #[must_use]
    pub fn new_binary_op(
        op: BinaryOp,
        lhs: DurativeActionFluentExpression,
        rhs: DurativeActionFluentExpression,
    ) -> Self {
        Self::binary_op(op, lhs, rhs)
    }

    /// Creates an operation over two or more operands, `(<multi-op> <f-exp-da> <f-exp-da>+)`.
    #[doc(alias = "new_multi_op")]
    #[must_use]
    pub fn multi_op<I: IntoIterator<Item = DurativeActionFluentExpression>>(
        op: MultiOp,
        lhs: DurativeActionFluentExpression,
        rhs: I,
    ) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    /// Creates an operation over two or more operands, `(<multi-op> <f-exp-da> <f-exp-da>+)`.
    #[must_use]
    pub fn new_multi_op<I: IntoIterator<Item = DurativeActionFluentExpression>>(
        op: MultiOp,
        lhs: DurativeActionFluentExpression,
        rhs: I,
    ) -> Self {
        Self::multi_op(op, lhs, rhs)
    }

    /// Creates a negation, `(- <f-exp-da>)`.
    #[doc(alias = "new_negative")]
    #[must_use]
    pub fn negative(value: DurativeActionFluentExpression) -> Self {
        Self::Negative(Box::new(value))
    }

    /// Creates a negation, `(- <f-exp-da>)`.
    #[must_use]
    pub fn new_negative(value: DurativeActionFluentExpression) -> Self {
        Self::negative(value)
    }

    /// Wraps a regular fluent expression.
    #[doc(alias = "new_f_exp")]
    #[must_use]
    pub const fn fluent_expression(f_head: FluentExpression) -> Self {
        Self::FluentExpression(f_head)
    }

    /// Wraps a regular fluent expression.
    #[must_use]
    pub fn new_f_exp(f_head: FluentExpression) -> Self {
        Self::fluent_expression(f_head)
    }
}

impl From<FluentExpression> for DurativeActionFluentExpression {
    fn from(value: FluentExpression) -> Self {
        DurativeActionFluentExpression::fluent_expression(value)
    }
}

/// Alias for [`DurativeActionFluentExpression`]; matches BNF `<f-exp-da>`.
#[deprecated(since = "0.2.0", note = "Use `DurativeActionFluentExpression` instead")]
pub type FExpDa = DurativeActionFluentExpression;
