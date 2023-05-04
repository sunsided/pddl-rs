//! Contains the [`ProblemConstraintsDef`] type.

use crate::PrefConGDs;
use std::ops::Deref;

/// A problem constraints definition; wraps a [`PrefConGD`].
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProblemConstraintsDef<'a>(PrefConGDs<'a>);

impl<'a> ProblemConstraintsDef<'a> {
    pub const fn new(gd: PrefConGDs<'a>) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PrefConGDs<'a> {
        &self.0
    }
}

impl<'a> PartialEq<PrefConGDs<'a>> for ProblemConstraintsDef<'a> {
    fn eq(&self, other: &PrefConGDs<'a>) -> bool {
        self.0.eq(other)
    }
}

impl<'a> From<PrefConGDs<'a>> for ProblemConstraintsDef<'a> {
    fn from(value: PrefConGDs<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> Deref for ProblemConstraintsDef<'a> {
    type Target = PrefConGDs<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Into<PrefConGDs<'a>> for ProblemConstraintsDef<'a> {
    fn into(self) -> PrefConGDs<'a> {
        self.0
    }
}
