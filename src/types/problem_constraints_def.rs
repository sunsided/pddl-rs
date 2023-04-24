//! Contains the [`ProblemConstraintsDef`] type.

use crate::types::PrefConGD;
use std::ops::Deref;

/// A problem constraints definition; wraps a [`PrefConGD`].
/// Requires [Constraints](crate::types::Requirement::Constraints).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProblemConstraintsDef<'a>(PrefConGD<'a>);

impl<'a> ProblemConstraintsDef<'a> {
    pub const fn new(gd: PrefConGD<'a>) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PrefConGD<'a> {
        &self.0
    }
}

impl<'a> PartialEq<PrefConGD<'a>> for ProblemConstraintsDef<'a> {
    fn eq(&self, other: &PrefConGD<'a>) -> bool {
        self.0.eq(other)
    }
}

impl<'a> From<PrefConGD<'a>> for ProblemConstraintsDef<'a> {
    fn from(value: PrefConGD<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> Deref for ProblemConstraintsDef<'a> {
    type Target = PrefConGD<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Into<PrefConGD<'a>> for ProblemConstraintsDef<'a> {
    fn into(self) -> PrefConGD<'a> {
        self.0
    }
}
