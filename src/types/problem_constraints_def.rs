//! Contains the [`ProblemConstraintsDef`] type.

use crate::PrefConGDs;
use std::ops::Deref;

/// A problem constraints definition; wraps a [`PrefConGD`].
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProblemConstraintsDef(PrefConGDs);

impl ProblemConstraintsDef {
    pub const fn new(gd: PrefConGDs) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PrefConGDs {
        &self.0
    }
}

impl PartialEq<PrefConGDs> for ProblemConstraintsDef {
    fn eq(&self, other: &PrefConGDs) -> bool {
        self.0.eq(other)
    }
}

impl From<PrefConGDs> for ProblemConstraintsDef {
    fn from(value: PrefConGDs) -> Self {
        Self::new(value)
    }
}

impl Deref for ProblemConstraintsDef {
    type Target = PrefConGDs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProblemConstraintsDef> for PrefConGDs {
    fn from(val: ProblemConstraintsDef) -> Self {
        val.0
    }
}
