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
    /// Creates a comparison against a duration value, e.g. `(= ?duration 5)`.
    #[doc(alias = "new_op")]
    #[must_use]
    pub const fn op(op: DurationOperator, value: DurationValue) -> Self {
        Self::Op(op, value)
    }

    /// Creates a comparison against a duration value. Same as [`op`](Self::op).
    #[must_use]
    pub fn new_op(op: DurationOperator, value: DurationValue) -> Self {
        Self::op(op, value)
    }

    /// Creates a constraint at a time point, e.g. `(at end (<= ?duration 5))`.
    #[doc(alias = "new_at")]
    #[must_use]
    pub fn at(time: TimeSpecifier, constraint: SimpleDurationConstraint) -> Self {
        Self::At(time, Box::new(constraint))
    }

    /// Creates a constraint at a time point. Same as [`at`](Self::at).
    #[must_use]
    pub fn new_at(time: TimeSpecifier, constraint: SimpleDurationConstraint) -> Self {
        Self::at(time, constraint)
    }
}

impl From<(DurationOperator, DurationValue)> for SimpleDurationConstraint {
    fn from(value: (DurationOperator, DurationValue)) -> Self {
        SimpleDurationConstraint::op(value.0, value.1)
    }
}

impl From<(TimeSpecifier, SimpleDurationConstraint)> for SimpleDurationConstraint {
    fn from(value: (TimeSpecifier, SimpleDurationConstraint)) -> Self {
        SimpleDurationConstraint::at(value.0, value.1)
    }
}
