//! Contains duration constraints via the [`DurationConstraint`] type.

use crate::types::iterators::FlatteningIntoIterator;
use crate::types::SimpleDurationConstraint;

/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum DurationConstraint<'a> {
    Single(SimpleDurationConstraint<'a>),
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities).
    All(Vec<SimpleDurationConstraint<'a>>),
}

impl<'a> IntoIterator for DurationConstraint<'a> {
    type Item = SimpleDurationConstraint<'a>;
    type IntoIter = FlatteningIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            DurationConstraint::Single(item) => FlatteningIntoIterator::new(item),
            DurationConstraint::All(vec) => FlatteningIntoIterator::new_vec(vec),
        }
    }
}

impl<'a> DurationConstraint<'a> {
    pub const fn new(constraint: SimpleDurationConstraint<'a>) -> Self {
        Self::Single(constraint)
    }

    pub fn new_all<I: IntoIterator<Item = SimpleDurationConstraint<'a>>>(constraints: I) -> Self {
        let vec: Vec<_> = constraints.into_iter().collect();
        debug_assert!(!vec.is_empty());
        Self::All(vec)
    }

    pub fn len(&self) -> usize {
        match self {
            DurationConstraint::Single(_) => 1,
            DurationConstraint::All(cs) => cs.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            DurationConstraint::Single(_) => false,
            DurationConstraint::All(xs) => xs.is_empty(),
        }
    }
}

impl<'a> From<SimpleDurationConstraint<'a>> for DurationConstraint<'a> {
    fn from(value: SimpleDurationConstraint<'a>) -> Self {
        DurationConstraint::new(value)
    }
}

impl<'a> FromIterator<SimpleDurationConstraint<'a>> for DurationConstraint<'a> {
    fn from_iter<T: IntoIterator<Item = SimpleDurationConstraint<'a>>>(iter: T) -> Self {
        DurationConstraint::new_all(iter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn flatten_with_single_element_works() {
        let (_, dc_a) = SimpleDurationConstraint::parse("(>= ?duration 1.23)").unwrap();

        let mut iter = DurationConstraint::new(dc_a).into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }

    #[test]
    fn flatten_with_many_elements_works() {
        let (_, dc_a) = SimpleDurationConstraint::parse("(>= ?duration 1.23)").unwrap();
        let (_, dc_b) = SimpleDurationConstraint::parse("(at end (<= ?duration 1.23))").unwrap();

        let mut iter = DurationConstraint::from_iter([dc_a, dc_b]).into_iter();
        assert!(iter.next().is_some());
        assert!(iter.next().is_some());
        assert!(iter.next().is_none());
    }
}
