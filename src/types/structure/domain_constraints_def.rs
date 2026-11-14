//! Contains the [`DomainConstraintsDef`] type.

use crate::types::ConstraintGoalDescription;
use std::ops::Deref;

/// A domain constraints definition; wraps a [`ConstraintGoalDescription`].
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
///
/// ## Usage
/// Used by [`Domain`](crate::Domain).
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct DomainConstraintsDef(ConstraintGoalDescription);

impl DomainConstraintsDef {
    /// Creates a new domain constraints definition, i.e. `(:constraints <con-GD>)`.
    #[must_use]
    pub const fn new(gd: ConstraintGoalDescription) -> Self {
        Self(gd)
    }

    /// Gets the value.
    #[must_use]
    pub const fn value(&self) -> &ConstraintGoalDescription {
        &self.0
    }
}

impl PartialEq<ConstraintGoalDescription> for DomainConstraintsDef {
    fn eq(&self, other: &ConstraintGoalDescription) -> bool {
        self.0.eq(other)
    }
}

impl From<ConstraintGoalDescription> for DomainConstraintsDef {
    fn from(value: ConstraintGoalDescription) -> Self {
        Self::new(value)
    }
}

impl Deref for DomainConstraintsDef {
    type Target = ConstraintGoalDescription;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DomainConstraintsDef> for ConstraintGoalDescription {
    fn from(val: DomainConstraintsDef) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn domain_constraints_def_from_con_gd() {
        let gd = ConstraintGoalDescription::default();
        let dcd: DomainConstraintsDef = gd.clone().into();
        check!(*dcd == gd);
    }

    #[test]
    fn domain_constraints_def_into_con_gd() {
        let gd = ConstraintGoalDescription::default();
        let dcd = DomainConstraintsDef::new(gd.clone());
        let back: ConstraintGoalDescription = dcd.into();
        check!(back == gd);
    }
}
