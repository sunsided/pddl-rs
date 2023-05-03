//! Contains the [`DomainConstraintsDef`] type.

use crate::types::ConGD;
use std::ops::Deref;

/// A domain constraints definition; wraps a [`ConGD`].
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DomainConstraintsDef<'a>(ConGD<'a>);

impl<'a> DomainConstraintsDef<'a> {
    pub const fn new(gd: ConGD<'a>) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &ConGD<'a> {
        &self.0
    }
}

impl<'a> PartialEq<ConGD<'a>> for DomainConstraintsDef<'a> {
    fn eq(&self, other: &ConGD<'a>) -> bool {
        self.0.eq(other)
    }
}

impl<'a> From<ConGD<'a>> for DomainConstraintsDef<'a> {
    fn from(value: ConGD<'a>) -> Self {
        Self::new(value)
    }
}

impl<'a> Deref for DomainConstraintsDef<'a> {
    type Target = ConGD<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> Into<ConGD<'a>> for DomainConstraintsDef<'a> {
    fn into(self) -> ConGD<'a> {
        self.0
    }
}
