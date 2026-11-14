//! Contains duration constraints via the [`DurationConstraint`] type.

use crate::types::iterators::FlatteningIntoIterator;
use crate::types::SimpleDurationConstraint;

/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum DurationConstraint {
    /// A single simple duration constraint.
    Single(SimpleDurationConstraint),
    /// ## Requirements
    /// Requires [Duration Inequalities](crate::Requirement::DurationInequalities).
    All(Vec<SimpleDurationConstraint>),
}

impl IntoIterator for DurationConstraint {
    type Item = SimpleDurationConstraint;
    type IntoIter = FlatteningIntoIterator<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            DurationConstraint::Single(item) => FlatteningIntoIterator::new(item),
            DurationConstraint::All(vec) => FlatteningIntoIterator::new_vec(vec),
        }
    }
}

impl DurationConstraint {
    /// Creates a constraint from a single simple duration constraint.
    pub const fn new(constraint: SimpleDurationConstraint) -> Self {
        Self::Single(constraint)
    }

    /// Creates a conjunction `(and ...)` of simple duration constraints.
    /// The iterator should not be empty; this is only checked in debug builds.
    #[doc(alias = "new_all")]
    pub fn all<I: IntoIterator<Item = SimpleDurationConstraint>>(constraints: I) -> Self {
        let vec: Vec<_> = constraints.into_iter().collect();
        debug_assert!(!vec.is_empty());
        Self::All(vec)
    }

    /// Creates a conjunction `(and ...)` of simple duration constraints. Same as [`all`](Self::all).
    pub fn new_all<I: IntoIterator<Item = SimpleDurationConstraint>>(constraints: I) -> Self {
        Self::all(constraints)
    }

    /// Returns the number of simple duration constraints.
    pub fn len(&self) -> usize {
        match self {
            DurationConstraint::Single(_) => 1,
            DurationConstraint::All(cs) => cs.len(),
        }
    }

    /// Returns `true` if there are no simple duration constraints.
    pub fn is_empty(&self) -> bool {
        match self {
            DurationConstraint::Single(_) => false,
            DurationConstraint::All(xs) => xs.is_empty(),
        }
    }
}

impl From<SimpleDurationConstraint> for DurationConstraint {
    fn from(value: SimpleDurationConstraint) -> Self {
        DurationConstraint::new(value)
    }
}

impl FromIterator<SimpleDurationConstraint> for DurationConstraint {
    fn from_iter<T: IntoIterator<Item = SimpleDurationConstraint>>(iter: T) -> Self {
        DurationConstraint::all(iter)
    }
}

#[cfg(all(test, feature = "parser"))]
mod tests {
    use super::*;
    use crate::Parser;
    use assert2::check;

    #[test]
    fn flatten_with_single_element_works() {
        let dc_a = SimpleDurationConstraint::from_str("(>= ?duration 1.23)").unwrap();

        let mut iter = DurationConstraint::new(dc_a).into_iter();
        check!(iter.next().is_some());
        check!(iter.next().is_none());
    }

    #[test]
    fn flatten_with_many_elements_works() {
        let dc_a = SimpleDurationConstraint::from_str("(>= ?duration 1.23)").unwrap();
        let dc_b = SimpleDurationConstraint::from_str("(at end (<= ?duration 1.23))").unwrap();

        let mut iter = DurationConstraint::from_iter([dc_a, dc_b]).into_iter();
        check!(iter.next().is_some());
        check!(iter.next().is_some());
        check!(iter.next().is_none());
    }
}
