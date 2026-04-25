//! Contains timed function expressions via the [`TimedFluentExpression`] type.

use crate::types::FluentExpression;

/// An f-exp-t.
///
/// ## Requirements
/// Requires [Continuous Effects](crate::Requirement::ContinuousEffects) and
/// [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`TimedEffect`](crate::TimedEffect).
#[doc(alias("f-exp-t"))]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum TimedFluentExpression {
    #[default]
    Now,
    Scaled(FluentExpression),
}

impl TimedFluentExpression {
    pub const fn new() -> Self {
        Self::Now
    }

    pub fn new_scaled(exp: FluentExpression) -> Self {
        Self::Scaled(exp)
    }
}

impl From<FluentExpression> for TimedFluentExpression {
    fn from(value: FluentExpression) -> Self {
        Self::Scaled(value)
    }
}

/// Alias for [`TimedFluentExpression`]; matches BNF `<f-exp-t>`.
#[deprecated(since = "0.2.0", note = "Use `TimedFluentExpression` instead")]
pub type FExpT = TimedFluentExpression;
