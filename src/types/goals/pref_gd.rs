//! Contains preference goal descriptions.

use crate::types::{GoalDescription, Preference};

/// A goal description that may be a preference, `<pref-GD>`.
///
/// ## Usage
/// Used by [`PreconditionGoalDescription`](crate::PreconditionGoalDescription).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PreferenceGoalDescription {
    /// A required goal, `<GD>`.
    Goal(GoalDescription),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Preference),
}

impl PreferenceGoalDescription {
    /// Creates a required goal from a [`GoalDescription`].
    #[must_use]
    pub const fn from_gd(gd: GoalDescription) -> Self {
        Self::Goal(gd)
    }

    /// Creates a preference goal from a [`Preference`].
    #[must_use]
    pub fn from_preference(pref: Preference) -> Self {
        Self::Preference(pref)
    }
}

impl From<GoalDescription> for PreferenceGoalDescription {
    fn from(value: GoalDescription) -> Self {
        PreferenceGoalDescription::from_gd(value)
    }
}

impl From<Preference> for PreferenceGoalDescription {
    fn from(value: Preference) -> Self {
        PreferenceGoalDescription::from_preference(value)
    }
}

/// Alias for [`PreferenceGoalDescription`]; matches BNF `<pref-GD>`.
#[deprecated(since = "0.2.0", note = "Use `PreferenceGoalDescription` instead")]
pub type PreferenceGD = PreferenceGoalDescription;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GoalDescription;
    use assert2::check;

    #[test]
    fn from_gd() {
        let gd = GoalDescription::and(Vec::new());
        let pref_gd = PreferenceGoalDescription::from_gd(gd.clone());
        check!(let PreferenceGoalDescription::Goal(_) = pref_gd);
    }

    #[test]
    fn from_preference() {
        let pref = crate::Preference::from(GoalDescription::and(Vec::new()));
        let pref_gd = PreferenceGoalDescription::from_preference(pref.clone());
        check!(let PreferenceGoalDescription::Preference(_) = pref_gd);
    }

    #[test]
    fn from_gd_trait() {
        let gd = GoalDescription::and(Vec::new());
        let pref_gd = PreferenceGoalDescription::from(gd);
        check!(let PreferenceGoalDescription::Goal(_) = pref_gd);
    }

    #[test]
    fn from_preference_trait() {
        let pref = crate::Preference::from(GoalDescription::and(Vec::new()));
        let pref_gd = PreferenceGoalDescription::from(pref);
        check!(let PreferenceGoalDescription::Preference(_) = pref_gd);
    }

    #[test]
    fn clone_works() {
        let gd = GoalDescription::and(Vec::new());
        let pref_gd = PreferenceGoalDescription::from_gd(gd);
        let clone = pref_gd.clone();
        check!(pref_gd == clone);
    }

    #[test]
    fn debug_impl() {
        let gd = GoalDescription::and(Vec::new());
        let pref_gd = PreferenceGoalDescription::from_gd(gd);
        let dbg = format!("{pref_gd:?}");
        check!(dbg.contains("Goal"));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias_exists() {
        fn _assert_alias(_: PreferenceGD) {}
    }
}
