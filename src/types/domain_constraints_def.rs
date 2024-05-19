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
pub struct DomainConstraintsDef(ConGD);

impl DomainConstraintsDef {
    pub const fn new(gd: ConGD) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &ConGD {
        &self.0
    }
}

impl PartialEq<ConGD> for DomainConstraintsDef {
    fn eq(&self, other: &ConGD) -> bool {
        self.0.eq(other)
    }
}

impl From<ConGD> for DomainConstraintsDef {
    fn from(value: ConGD) -> Self {
        Self::new(value)
    }
}

impl Deref for DomainConstraintsDef {
    type Target = ConGD;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DomainConstraintsDef> for ConGD {
    fn from(val: DomainConstraintsDef) -> Self {
        val.0
    }
}
