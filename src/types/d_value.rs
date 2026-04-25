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
    #[doc(alias = "new_number")]
    pub fn number<I: Into<Number>>(number: I) -> Self {
        Self::Number(number.into())
    }

    pub fn new_number<I: Into<Number>>(number: I) -> Self {
        Self::number(number)
    }

    #[doc(alias = "new_f_exp")]
    pub fn fluent_expression(exp: FluentExpression) -> Self {
        Self::FluentExpression(exp)
    }

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

    #[test]
    fn duration_value_from_number() {
        let n = Number::from(5);
        let dv: DurationValue = n.into();
        assert_eq!(dv, DurationValue::number(n));
    }

    #[test]
    fn duration_value_from_f_exp() {
        let exp = FluentExpression::number(Number::from(10));
        let dv: DurationValue = exp.clone().into();
        assert_eq!(dv, DurationValue::fluent_expression(exp));
    }
}
