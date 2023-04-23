//! Contains the [`DValue`] type.

use crate::types::{FExp, Number};

/// A d-value.
#[derive(Debug, Clone, PartialEq)]
pub enum DValue<'a> {
    Number(Number),
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    FExp(FExp<'a>),
}

impl<'a> DValue<'a> {
    pub fn new_number<I: Into<Number>>(number: I) -> Self {
        Self::Number(number.into())
    }

    pub fn new_f_exp(exp: FExp<'a>) -> Self {
        Self::FExp(exp)
    }
}

impl<'a> From<Number> for DValue<'a> {
    fn from(value: Number) -> Self {
        Self::Number(value)
    }
}

impl<'a> From<FExp<'a>> for DValue<'a> {
    fn from(value: FExp<'a>) -> Self {
        Self::FExp(value)
    }
}
