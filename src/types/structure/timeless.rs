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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Timeless(Vec<NameLiteral>);

impl Timeless {
    /// Creates a new set of timeless literals, i.e. the content of `(:timeless <literal (name)>+)`.
    #[must_use]
    pub fn new(literal: Vec<NameLiteral>) -> Self {
        Self(literal)
    }

    /// Gets the literals.
    #[must_use]
    pub fn values(&self) -> &[NameLiteral] {
        self.0.as_slice()
    }
}

impl FromIterator<NameLiteral> for Timeless {
    fn from_iter<T: IntoIterator<Item = NameLiteral>>(iter: T) -> Self {
        Timeless::new(iter.into_iter().collect())
    }
}

impl Deref for Timeless {
    type Target = [NameLiteral];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
