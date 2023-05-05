//! Contains the [`DurationValue`] type.

use crate::types::{FExp, Number};

/// A duration value, either a [`Number`] or an [`FExp`](FExp).
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
    FExp(FExp),
}

impl DurationValue {
    pub fn new_number<I: Into<Number>>(number: I) -> Self {
        Self::Number(number.into())
    }

    pub fn new_f_exp(exp: FExp) -> Self {
        Self::FExp(exp)
    }
}

impl From<Number> for DurationValue {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<FExp> for DurationValue {
    fn from(value: FExp) -> Self {
        Self::FExp(value)
    }
}
