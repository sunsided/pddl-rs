//! Contains preference goal definitions.

use crate::types::{GoalDefinition, Preference};

/// A preferred goal definition.
///
/// ## Usage
/// Used by [`PreconditionGoalDefinition`](crate::PreconditionGoalDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceGoalDefinition {
    Goal(GoalDefinition),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Preference),
}

impl PreferenceGoalDefinition {
    pub const fn from_gd(gd: GoalDefinition) -> Self {
        Self::Goal(gd)
    }

    pub fn from_preference(pref: Preference) -> Self {
        Self::Preference(pref)
    }
}

impl From<GoalDefinition> for PreferenceGoalDefinition {
    fn from(value: GoalDefinition) -> Self {
        PreferenceGoalDefinition::from_gd(value)
    }
}

impl From<Preference> for PreferenceGoalDefinition {
    fn from(value: Preference) -> Self {
        PreferenceGoalDefinition::from_preference(value)
    }
}

/// Alias for [`PreferenceGoalDefinition`]; matches BNF `<pref-GD>`.
#[deprecated(since = "0.2.0", note = "Use `PreferenceGoalDefinition` instead")]
pub type PreferenceGD = PreferenceGoalDefinition;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GoalDefinition;

    #[test]
    fn from_gd() {
        let gd = GoalDefinition::and(Vec::new());
        let pref_gd = PreferenceGoalDefinition::from_gd(gd.clone());
        assert!(matches!(pref_gd, PreferenceGoalDefinition::Goal(_)));
    }

    #[test]
    fn from_preference() {
        let pref = crate::Preference::from(GoalDefinition::and(Vec::new()));
        let pref_gd = PreferenceGoalDefinition::from_preference(pref.clone());
        assert!(matches!(pref_gd, PreferenceGoalDefinition::Preference(_)));
    }

    #[test]
    fn from_gd_trait() {
        let gd = GoalDefinition::and(Vec::new());
        let pref_gd = PreferenceGoalDefinition::from(gd);
        assert!(matches!(pref_gd, PreferenceGoalDefinition::Goal(_)));
    }

    #[test]
    fn from_preference_trait() {
        let pref = crate::Preference::from(GoalDefinition::and(Vec::new()));
        let pref_gd = PreferenceGoalDefinition::from(pref);
        assert!(matches!(pref_gd, PreferenceGoalDefinition::Preference(_)));
    }

    #[test]
    fn clone_works() {
        let gd = GoalDefinition::and(Vec::new());
        let pref_gd = PreferenceGoalDefinition::from_gd(gd);
        let clone = pref_gd.clone();
        assert_eq!(pref_gd, clone);
    }

    #[test]
    fn debug_impl() {
        let gd = GoalDefinition::and(Vec::new());
        let pref_gd = PreferenceGoalDefinition::from_gd(gd);
        let dbg = format!("{pref_gd:?}");
        assert!(dbg.contains("Goal"));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias_exists() {
        fn _assert_alias(_: PreferenceGD) {}
    }
}
