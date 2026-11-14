//! Contains the [`DurationValue`] type.

use crate::types::{FluentExpression, Number};

/// A duration value, either a [`Number`] or a [`FluentExpression`].
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
    /// Creates a duration value from a number.
    #[doc(alias = "new_number")]
    pub fn number<I: Into<Number>>(number: I) -> Self {
        Self::Number(number.into())
    }

    /// Creates a duration value from a number. Same as [`number`](Self::number).
    pub fn new_number<I: Into<Number>>(number: I) -> Self {
        Self::number(number)
    }

    /// Creates a duration value from a fluent expression.
    #[doc(alias = "new_f_exp")]
    pub fn fluent_expression(exp: FluentExpression) -> Self {
        Self::FluentExpression(exp)
    }

    /// Creates a duration value from a fluent expression. Same as [`fluent_expression`](Self::fluent_expression).
    pub fn new_f_exp(exp: FluentExpression) -> Self {
        Self::fluent_expression(exp)
    }
}

impl From<Number> for DurationValue {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl From<FluentExpression> for DurationValue {
    fn from(value: FluentExpression) -> Self {
        Self::fluent_expression(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn duration_value_from_number() {
        let n = Number::from(5);
        let dv: DurationValue = n.into();
        check!(dv == DurationValue::number(n));
    }

    #[test]
    fn duration_value_from_f_exp() {
        let exp = FluentExpression::number(Number::from(10));
        let dv: DurationValue = exp.clone().into();
        check!(dv == DurationValue::fluent_expression(exp));
    }
}
