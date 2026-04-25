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
    pub const fn new_duration() -> Self {
        Self::Duration
    }

    pub fn new_binary_op(
        op: BinaryOp,
        lhs: DurativeActionFluentExpression,
        rhs: DurativeActionFluentExpression,
    ) -> Self {
        Self::BinaryOp(op, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_multi_op<I: IntoIterator<Item = DurativeActionFluentExpression>>(
        op: MultiOp,
        lhs: DurativeActionFluentExpression,
        rhs: I,
    ) -> Self {
        Self::MultiOp(op, Box::new(lhs), rhs.into_iter().collect())
    }

    pub fn new_negative(value: DurativeActionFluentExpression) -> Self {
        Self::Negative(Box::new(value))
    }

    pub const fn new_f_exp(f_head: FluentExpression) -> Self {
        Self::FluentExpression(f_head)
    }
}

impl From<FluentExpression> for DurativeActionFluentExpression {
    fn from(value: FluentExpression) -> Self {
        DurativeActionFluentExpression::new_f_exp(value)
    }
}

/// Alias for [`DurativeActionFluentExpression`]; matches BNF `<f-exp-da>`.
#[deprecated(since = "0.2.0", note = "Use `DurativeActionFluentExpression` instead")]
pub type FExpDa = DurativeActionFluentExpression;
