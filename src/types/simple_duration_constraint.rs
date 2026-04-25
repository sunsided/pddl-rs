//! Contains the [`SimpleDurationConstraint`] type.

use crate::types::{DurationOperator, DurationValue, TimeSpecifier};

/// A simple duration constraint.
///
/// ## Usage
/// Used by [`SimpleDurationConstraint`] itself, as well as [`DurationConstraint`](crate::DurationConstraint).
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleDurationConstraint {
    /// A comparison operation against a duration value.
    Op(DurationOperator, DurationValue),
    /// A specific time at or after which a constraint applies.
    At(TimeSpecifier, Box<SimpleDurationConstraint>),
}

impl SimpleDurationConstraint {
    pub const fn new_op(op: DurationOperator, value: DurationValue) -> Self {
        Self::Op(op, value)
    }

    pub fn new_at(time: TimeSpecifier, constraint: SimpleDurationConstraint) -> Self {
        Self::At(time, Box::new(constraint))
    }
}

impl From<(DurationOperator, DurationValue)> for SimpleDurationConstraint {
    fn from(value: (DurationOperator, DurationValue)) -> Self {
        SimpleDurationConstraint::new_op(value.0, value.1)
    }
}

impl From<(TimeSpecifier, SimpleDurationConstraint)> for SimpleDurationConstraint {
    fn from(value: (TimeSpecifier, SimpleDurationConstraint)) -> Self {
        SimpleDurationConstraint::new_at(value.0, value.1)
    }
}
