//! Contains the [`ProblemConstraintsDef`] type.

use crate::PreferenceConstraintGoalDescriptions;
use std::ops::Deref;

/// A problem constraints definition; wraps a [`PreferenceConstraintGoalDescriptions`].
///
/// ## Requirements
/// Requires [Constraints](crate::Requirement::Constraints).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ProblemConstraintsDef(PreferenceConstraintGoalDescriptions);

impl ProblemConstraintsDef {
    /// Creates a new problem constraints definition, i.e. `(:constraints <pref-con-GD>)`.
    #[must_use]
    pub const fn new(gd: PreferenceConstraintGoalDescriptions) -> Self {
        Self(gd)
    }

    /// Gets the value.
    #[must_use]
    pub const fn value(&self) -> &PreferenceConstraintGoalDescriptions {
        &self.0
    }
}

impl PartialEq<PreferenceConstraintGoalDescriptions> for ProblemConstraintsDef {
    fn eq(&self, other: &PreferenceConstraintGoalDescriptions) -> bool {
        self.0.eq(other)
    }
}

impl From<PreferenceConstraintGoalDescriptions> for ProblemConstraintsDef {
    fn from(value: PreferenceConstraintGoalDescriptions) -> Self {
        Self::new(value)
    }
}

impl Deref for ProblemConstraintsDef {
    type Target = PreferenceConstraintGoalDescriptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProblemConstraintsDef> for PreferenceConstraintGoalDescriptions {
    fn from(val: ProblemConstraintsDef) -> Self {
        val.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;

    #[test]
    fn default_is_empty() {
        let def = ProblemConstraintsDef::default();
        check!(def.is_empty());
    }

    #[test]
    fn new_works() {
        let defs = PreferenceConstraintGoalDescriptions::default();
        let def = ProblemConstraintsDef::new(defs.clone());
        check!(*def.value() == defs);
    }

    #[test]
    fn from_works() {
        let defs = PreferenceConstraintGoalDescriptions::default();
        let def = ProblemConstraintsDef::from(defs.clone());
        check!(def == ProblemConstraintsDef::new(defs));
    }

    #[test]
    fn deref_works() {
        let def = ProblemConstraintsDef::default();
        let inner: &PreferenceConstraintGoalDescriptions = &def;
        check!(inner.is_empty());
    }

    #[test]
    fn into_works() {
        let defs = PreferenceConstraintGoalDescriptions::default();
        let def = ProblemConstraintsDef::new(defs.clone());
        let result: PreferenceConstraintGoalDescriptions = def.into();
        check!(result == defs);
    }

    #[test]
    fn clone_works() {
        let def = ProblemConstraintsDef::default();
        let clone = def.clone();
        check!(def == clone);
    }
}
