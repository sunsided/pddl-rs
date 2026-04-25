//! Contains durative function expressions via the [`DurativeActionFluentExpression`] type.

use crate::types::{AssignOp, BinaryOp, FluentExpression, FunctionHead, MultiOp};

/// ## Usage
/// Used by [`DurativeActionFluentExpression`] itself, as well as [`DurativeActionFunctionAssignment`](crate::DurativeActionFunctionAssignment).
#[doc(alias("f-exp-da"))]
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionFluentExpression {
    Assign(AssignOp, FunctionHead, Box<DurativeActionFluentExpression>),
    BinaryOp(
        BinaryOp,
        Box<DurativeActionFluentExpression>,
        Box<DurativeActionFluentExpression>,
    ),
    MultiOp(
        MultiOp,
        Box<DurativeActionFluentExpression>,
        Vec<DurativeActionFluentExpression>,
    ),
    Negative(Box<DurativeActionFluentExpression>),
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities).
    Duration,
    FluentExpression(FluentExpression),
}

impl DurativeActionFluentExpression {
    #[doc(alias = "new_duration")]
    pub const fn duration() -> Self {
        Self::Duration
    }

    pub fn new_duration() -> Self {
        Self::duration()
    }

    #[doc(alias = "new_binary_op")]
    pub fn binary_op(
        op: BinaryOp,
        lhs: DurativeActionFluentExpression,
        rhs: DurativeActionFluentExpression,
    ) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_binary_op(
        op: BinaryOp,
        lhs: DurativeActionFluentExpression,
        rhs: DurativeActionFluentExpression,
    ) -> Self {
        Self::binary_op(op, lhs, rhs)
    }

    #[doc(alias = "new_multi_op")]
    pub fn multi_op<I: IntoIterator<Item = DurativeActionFluentExpression>>(
        op: MultiOp,
        lhs: DurativeActionFluentExpression,
        rhs: I,
    ) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    pub fn new_multi_op<I: IntoIterator<Item = DurativeActionFluentExpression>>(
        op: MultiOp,
        lhs: DurativeActionFluentExpression,
        rhs: I,
    ) -> Self {
        Self::multi_op(op, lhs, rhs)
    }

    #[doc(alias = "new_negative")]
    pub fn negative(value: DurativeActionFluentExpression) -> Self {
        Self::Negative(Box::new(value))
    }

    pub fn new_negative(value: DurativeActionFluentExpression) -> Self {
        Self::negative(value)
    }

    #[doc(alias = "new_f_exp")]
    pub const fn fluent_expression(f_head: FluentExpression) -> Self {
        Self::FluentExpression(f_head)
    }

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
