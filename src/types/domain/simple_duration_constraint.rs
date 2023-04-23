//! Contains the [`SimpleDurationConstraint`] type.

use crate::types::domain::{DOp, DValue, TimeSpecifier};

/// A simple duration constraint.
#[derive(Debug, Clone, PartialEq)]
pub enum SimpleDurationConstraint<'a> {
    Op(DOp, DValue<'a>),
    At(TimeSpecifier, Box<SimpleDurationConstraint<'a>>),
}

impl<'a> SimpleDurationConstraint<'a> {
    pub const fn new_op(op: DOp, value: DValue<'a>) -> Self {
        Self::Op(op, value)
    }

    pub fn new_at(time: TimeSpecifier, constraint: SimpleDurationConstraint<'a>) -> Self {
        Self::At(time, Box::new(constraint))
    }
}

impl<'a> From<(DOp, DValue<'a>)> for SimpleDurationConstraint<'a> {
    fn from(value: (DOp, DValue<'a>)) -> Self {
        SimpleDurationConstraint::new_op(value.0, value.1)
    }
}

impl<'a> From<(TimeSpecifier, SimpleDurationConstraint<'a>)> for SimpleDurationConstraint<'a> {
    fn from(value: (TimeSpecifier, SimpleDurationConstraint<'a>)) -> Self {
        SimpleDurationConstraint::new_at(value.0, value.1)
    }
}
