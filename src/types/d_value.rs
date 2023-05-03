//! Contains the [`DurationValue`] type.

use crate::types::{FExp, Number};

/// A duration value, either a [Number] or an [FExp](FExp).
///
/// ## Usage
/// Used by [`SimpleDurationConstraint`](crate::SimpleDurationConstraint).
#[derive(Debug, Clone, PartialEq)]
pub enum DurationValue<'a> {
    /// A numerical value.
    Number(Number),
    /// A function expression that produces the duration value.
    /// ## Requirements
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    FExp(FExp<'a>),
}

impl<'a> DurationValue<'a> {
    pub fn new_number<I: Into<Number>>(number: I) -> Self {
        Self::Number(number.into())
    }

    pub fn new_f_exp(exp: FExp<'a>) -> Self {
        Self::FExp(exp)
    }
}

impl<'a> From<Number> for DurationValue<'a> {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl<'a> From<FExp<'a>> for DurationValue<'a> {
    fn from(value: FExp<'a>) -> Self {
        Self::FExp(value)
    }
}
