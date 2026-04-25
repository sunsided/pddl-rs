//! Contains the [`DomainConstraintsDef`] type.

use crate::types::ConstraintGoalDefinition;
use std::ops::Deref;

/// A domain constraints definition; wraps a [`ConstraintGoalDefinition`].
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct DomainConstraintsDef(ConstraintGoalDefinition);

impl DomainConstraintsDef {
    pub const fn new(gd: ConstraintGoalDefinition) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &ConstraintGoalDefinition {
        &self.0
    }
}

impl PartialEq<ConstraintGoalDefinition> for DomainConstraintsDef {
    fn eq(&self, other: &ConstraintGoalDefinition) -> bool {
        self.0.eq(other)
    }
}

impl From<ConstraintGoalDefinition> for DomainConstraintsDef {
    fn from(value: ConstraintGoalDefinition) -> Self {
        Self::new(value)
    }
}

impl Deref for DomainConstraintsDef {
    type Target = ConstraintGoalDefinition;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DomainConstraintsDef> for ConstraintGoalDefinition {
    fn from(val: DomainConstraintsDef) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_constraints_def_from_con_gd() {
        let gd = ConstraintGoalDefinition::default();
        let dcd: DomainConstraintsDef = gd.clone().into();
        assert_eq!(*dcd, gd);
    }

    #[test]
    fn domain_constraints_def_into_con_gd() {
        let gd = ConstraintGoalDefinition::default();
        let dcd = DomainConstraintsDef::new(gd.clone());
        let back: ConstraintGoalDefinition = dcd.into();
        assert_eq!(back, gd);
    }
}
