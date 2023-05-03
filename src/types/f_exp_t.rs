//! Contains timed function expressions via the [`FExpT`] type.

use crate::types::FExp;

/// An f-exp-t.
///
/// ## Requirements
/// Requires [Continuous Effects](crate::Requirement::ContinuousEffects) and
/// [Numeric Fluents](crate::Requirement::NumericFluents).
///
/// ## Usage
/// Used by [`TimedEffect`](crate::TimedEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum FExpT<'a> {
    Now,
    Scaled(FExp<'a>),
}

impl<'a> FExpT<'a> {
    pub const fn new() -> Self {
        Self::Now
    }

    pub fn new_scaled(exp: FExp<'a>) -> Self {
        Self::Scaled(exp)
    }
}

impl<'a> Default for FExpT<'a> {
    fn default() -> Self {
        Self::Now
    }
}

impl<'a> From<FExp<'a>> for FExpT<'a> {
    fn from(value: FExp<'a>) -> Self {
        Self::Scaled(value)
    }
}
