//! Contains the [`DValue`] type.

use crate::types::domain::FExp;

/// A d-value.
#[derive(Debug, Clone, PartialEq)]
pub enum DValue<'a> {
    Number(f32),
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    FExp(FExp<'a>),
}

impl<'a> DValue<'a> {
    pub const fn new_number(number: f32) -> Self {
        Self::Number(number)
    }

    pub fn new_f_exp(exp: FExp<'a>) -> Self {
        Self::FExp(exp)
    }
}

impl<'a> From<f32> for DValue<'a> {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}

impl<'a> From<FExp<'a>> for DValue<'a> {
    fn from(value: FExp<'a>) -> Self {
        Self::FExp(value)
    }
}
