//! Contains duration constraints via the [`DurationConstraint`] type.

use crate::types::SimpleDurationConstraint;

/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum DurationConstraint<'a> {
    Simple(SimpleDurationConstraint<'a>),
    /// ## Requirements
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

    pub fn len(&self) -> usize {
        match self {
            DurationConstraint::Simple(_) => 1,
            DurationConstraint::And(cs) => cs.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            DurationConstraint::Simple(_) => false,
            DurationConstraint::And(xs) => xs.is_empty(),
        }
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
