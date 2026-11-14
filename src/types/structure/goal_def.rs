//! Contains the [`ProblemGoalDescription`] type.

use crate::types::PreconditionGoalDescriptions;
use crate::PreconditionGoalDescription;
use std::ops::Deref;

/// A problem goal description; wraps a [`PreconditionGoalDescriptions`].
///
/// # BNF
/// Corresponds to the `(:goal ...)` section of a PDDL problem definition.
///
/// ## Usage
/// Used by [`Problem`](crate::Problem).
#[doc(alias("goal"))]
#[derive(Debug, Clone, PartialEq)]
pub struct ProblemGoalDescription(PreconditionGoalDescriptions);

impl ProblemGoalDescription {
    /// Creates a new problem goal description, i.e. `(:goal <pre-GD>)`.
    pub const fn new(gd: PreconditionGoalDescriptions) -> Self {
        Self(gd)
    }

    /// Gets the value.
    pub const fn value(&self) -> &PreconditionGoalDescriptions {
        &self.0
    }
}

impl PartialEq<PreconditionGoalDescriptions> for ProblemGoalDescription {
    fn eq(&self, other: &PreconditionGoalDescriptions) -> bool {
        self.0.eq(other)
    }
}

impl From<PreconditionGoalDescriptions> for ProblemGoalDescription {
    fn from(value: PreconditionGoalDescriptions) -> Self {
        Self::new(value)
    }
}

impl From<PreconditionGoalDescription> for ProblemGoalDescription {
    fn from(value: PreconditionGoalDescription) -> Self {
        Self::new(PreconditionGoalDescriptions::from(value))
    }
}

impl FromIterator<PreconditionGoalDescription> for ProblemGoalDescription {
    fn from_iter<T: IntoIterator<Item = PreconditionGoalDescription>>(iter: T) -> Self {
        ProblemGoalDescription::new(PreconditionGoalDescriptions::from_iter(iter))
    }
}

impl Deref for ProblemGoalDescription {
    type Target = PreconditionGoalDescriptions;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<ProblemGoalDescription> for PreconditionGoalDescriptions {
    fn from(val: ProblemGoalDescription) -> Self {
        val.0
    }
}

/// Alias for [`ProblemGoalDescription`].
#[deprecated(since = "0.2.0", note = "Use `ProblemGoalDescription` instead")]
pub type GoalDef = ProblemGoalDescription;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GoalDescription;
    use assert2::check;

    #[test]
    fn problem_goal_description_from_precondition_goal_descriptions() {
        let defs = PreconditionGoalDescriptions::new(Vec::new());
        let pgd: ProblemGoalDescription = defs.clone().into();
        check!(*pgd.value() == defs);
    }

    #[test]
    fn problem_goal_description_from_precondition_goal_description() {
        let pref_gd = crate::PreferenceGoalDescription::from_gd(GoalDescription::and(Vec::new()));
        let pre_gd = PreconditionGoalDescription::preference(pref_gd);
        let pgd: ProblemGoalDescription = pre_gd.into();
        check!(pgd.len() == 1);
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias_exists() {
        fn _assert_alias(_: GoalDef) {}
    }
}
