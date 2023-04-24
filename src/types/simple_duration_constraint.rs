//! Contains the [`SimpleDurationConstraint`] type.

use crate::types::{DOp, DurationValue, TimeSpecifier};

/// A simple duration constraint.
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleDurationConstraint<'a> {
    /// A comparison operation against a duration value.
    Op(DOp, DurationValue<'a>),
    /// A specific time at or after which a constraint applies.
    At(TimeSpecifier, Box<SimpleDurationConstraint<'a>>),
}

impl<'a> SimpleDurationConstraint<'a> {
    pub const fn new_op(op: DOp, value: DurationValue<'a>) -> Self {
        Self::Op(op, value)
    }

    pub fn new_at(time: TimeSpecifier, constraint: SimpleDurationConstraint<'a>) -> Self {
        Self::At(time, Box::new(constraint))
    }
}

impl<'a> From<(DOp, DurationValue<'a>)> for SimpleDurationConstraint<'a> {
    fn from(value: (DOp, DurationValue<'a>)) -> Self {
        SimpleDurationConstraint::new_op(value.0, value.1)
    }
}

impl<'a> From<(TimeSpecifier, SimpleDurationConstraint<'a>)> for SimpleDurationConstraint<'a> {
    fn from(value: (TimeSpecifier, SimpleDurationConstraint<'a>)) -> Self {
        SimpleDurationConstraint::new_at(value.0, value.1)
    }
}
