//! Contains the [`DurationValue`] type.

use crate::types::{FluentExpression, Number};

/// A duration value, either a [`Number`] or an [`FluentExpression`](FluentExpression).
///
/// ## Usage
/// Used by [`SimpleDurationConstraint`](crate::SimpleDurationConstraint).
#[derive(Debug, Clone, PartialEq)]
pub enum DurationValue {
    /// A numerical value.
    Number(Number),
    /// A function expression that produces the duration value.
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    FluentExpression(FluentExpression),
}

impl DurationValue {
    pub fn new_number<I: Into<Number>>(number: I) -> Self {
        Self::Number(number.into())
    }

    pub fn new_f_exp(exp: FluentExpression) -> Self {
        Self::FluentExpression(exp)
    }
}

impl From<Number> for DurationValue {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<FluentExpression> for DurationValue {
    fn from(value: FluentExpression) -> Self {
        Self::FluentExpression(value)
    }
}
