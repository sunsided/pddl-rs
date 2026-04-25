//! Contains preferences.

use crate::types::{GoalDefinition, PreferenceName};

/// A preference.
///
/// ## Requirements
/// Requires [Preferences](crate::Requirement::Preferences).
///
/// ## Usage
/// Used by [`PreferenceGoalDefinition`](crate::PreferenceGoalDefinition).
#[derive(Debug, Clone, PartialEq)]
pub struct Preference(Option<PreferenceName>, GoalDefinition); // TODO: A similar type is used for PrefConGD

impl Preference {
    pub const fn new(name: Option<PreferenceName>, gd: GoalDefinition) -> Self {
        Self(name, gd)
    }

    /// Gets the optional preference name.
    pub fn name(&self) -> &Option<PreferenceName> {
        &self.0
    }

    /// Gets the goal definition.
    pub fn goal(&self) -> &GoalDefinition {
        &self.1
    }

    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }
}

impl From<(Option<PreferenceName>, GoalDefinition)> for Preference {
    fn from(value: (Option<PreferenceName>, GoalDefinition)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(PreferenceName, GoalDefinition)> for Preference {
    fn from(value: (PreferenceName, GoalDefinition)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}

impl From<GoalDefinition> for Preference {
    fn from(value: GoalDefinition) -> Self {
        Self::new(None, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GoalDefinition, Name, Term};

    #[test]
    fn new_with_name() {
        let name = PreferenceName::string("pref1");
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::new(Some(name.clone()), gd.clone());
        assert_eq!(pref.name(), &Some(name));
        assert_eq!(pref.goal(), &gd);
    }

    #[test]
    fn new_without_name() {
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::new(None, gd.clone());
        assert_eq!(pref.name(), &None);
        assert_eq!(pref.goal(), &gd);
    }

    #[test]
    fn from_tuple_with_option() {
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::from((None::<PreferenceName>, gd.clone()));
        assert_eq!(pref.name(), &None);
        assert_eq!(pref.goal(), &gd);
    }

    #[test]
    fn from_tuple_with_name() {
        let name = PreferenceName::string("my_pref");
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::from((name.clone(), gd.clone()));
        assert_eq!(pref.name(), &Some(name));
        assert_eq!(pref.goal(), &gd);
    }

    #[test]
    fn from_gd_only() {
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::from(gd.clone());
        assert_eq!(pref.name(), &None);
        assert_eq!(pref.goal(), &gd);
    }

    #[test]
    fn is_empty_true() {
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::new(None, gd);
        assert!(pref.is_empty());
    }

    #[test]
    fn is_empty_false() {
        use crate::AtomicFormula;
        let af = AtomicFormula::<Term>::predicate(
            crate::Predicate::new(Name::new("on")),
            vec![
                Term::new_name(Name::new("a")),
                Term::new_name(Name::new("b")),
            ],
        );
        let gd = GoalDefinition::AtomicFormula(af);
        let pref = Preference::new(None, gd);
        assert!(!pref.is_empty());
    }

    #[test]
    fn clone_works() {
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::new(None, gd);
        let clone = pref.clone();
        assert_eq!(pref, clone);
    }

    #[test]
    fn debug_impl() {
        let gd = GoalDefinition::and(Vec::new());
        let pref = Preference::new(None, gd);
        let dbg = format!("{pref:?}");
        assert!(dbg.contains("Preference"));
    }
}
