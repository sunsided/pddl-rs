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
pub enum FExpT {
    Now,
    Scaled(FExp),
}

impl FExpT {
    pub const fn new() -> Self {
        Self::Now
    }

    pub fn new_scaled(exp: FExp) -> Self {
        Self::Scaled(exp)
    }
}

impl Default for FExpT {
    fn default() -> Self {
        Self::Now
    }
}

impl From<FExp> for FExpT {
    fn from(value: FExp) -> Self {
        Self::Scaled(value)
    }
}
