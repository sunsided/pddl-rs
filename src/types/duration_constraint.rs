//! Contains the [`DurationConstraint`] type.

use crate::types::SimpleDurationConstraint;

#[derive(Debug, Clone, PartialEq)]
pub enum DurationConstraint<'a> {
    Simple(SimpleDurationConstraint<'a>),
    /// Requires [DurationInequalities](crate::types::Requirement::DurationInequalities).
    And(Vec<SimpleDurationConstraint<'a>>),
}

impl<'a> DurationConstraint<'a> {
    pub const fn new_simple(constraint: SimpleDurationConstraint<'a>) -> Self {
        Self::Simple(constraint)
    }

    pub fn new_all<I: IntoIterator<Item = SimpleDurationConstraint<'a>>>(constraints: I) -> Self {
        let vec: Vec<_> = constraints.into_iter().collect();
        debug_assert!(!vec.is_empty());
        Self::And(vec)
    }
}

impl<'a> From<SimpleDurationConstraint<'a>> for DurationConstraint<'a> {
    fn from(value: SimpleDurationConstraint<'a>) -> Self {
        DurationConstraint::new_simple(value)
    }
}

impl<'a> FromIterator<SimpleDurationConstraint<'a>> for DurationConstraint<'a> {
    fn from_iter<T: IntoIterator<Item = SimpleDurationConstraint<'a>>>(iter: T) -> Self {
        DurationConstraint::new_all(iter)
    }
}
