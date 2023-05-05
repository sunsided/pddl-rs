//! Contains the [`Timeless`] type.

use crate::types::NameLiteral;
use std::ops::Deref;

/// A timeless predicate.
///
/// A timeless predicate is a predicate which is always true and cannot be changed by any action
/// in the domain. Under the “closed world” assumption, anything not specified as true
/// is considered false and timeless predicates are one possibility of addressing this.
///
/// ## PDDL Version
/// This is a PDDL 1.2 construct. It was removed in later versions of PDDL.
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Timeless(Vec<NameLiteral>);

impl Timeless {
    pub fn new(literal: Vec<NameLiteral>) -> Self {
        Self(literal)
    }

    /// Gets the literals.
    pub fn values(&self) -> &[NameLiteral] {
        &self.0.as_slice()
    }
}

impl FromIterator<NameLiteral> for Timeless {
    fn from_iter<T: IntoIterator<Item = NameLiteral>>(iter: T) -> Self {
        Timeless::new(iter.into_iter().collect())
    }
}

impl Deref for Timeless {
    type Target = [NameLiteral];

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
