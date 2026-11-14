//! Contains preferences.

use crate::types::{GoalDescription, PreferenceName};

/// A preference.
///
/// ## Requirements
/// Requires [Preferences](crate::Requirement::Preferences).
///
/// ## Usage
/// Used by [`PreferenceGoalDescription`](crate::PreferenceGoalDescription).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Preference(Option<PreferenceName>, GoalDescription); // TODO: A similar type is used for PrefConGD

impl Preference {
    /// Creates a preference `(preference [<pref-name>] <GD>)` with an optional name.
    #[must_use]
    pub const fn new(name: Option<PreferenceName>, gd: GoalDescription) -> Self {
        Self(name, gd)
    }

    /// Gets the optional preference name.
    #[must_use]
    pub fn name(&self) -> &Option<PreferenceName> {
        &self.0
    }

    /// Gets the goal description.
    #[must_use]
    pub fn goal(&self) -> &GoalDescription {
        &self.1
    }

    /// Returns `true` if the goal description is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.1.is_empty()
    }
}

impl From<(Option<PreferenceName>, GoalDescription)> for Preference {
    fn from(value: (Option<PreferenceName>, GoalDescription)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl From<(PreferenceName, GoalDescription)> for Preference {
    fn from(value: (PreferenceName, GoalDescription)) -> Self {
        Self::new(Some(value.0), value.1)
    }
}

impl From<GoalDescription> for Preference {
    fn from(value: GoalDescription) -> Self {
        Self::new(None, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{GoalDescription, Name, Term};
    use assert2::check;

    #[test]
    fn new_with_name() {
        let name = PreferenceName::string("pref1");
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::new(Some(name.clone()), gd.clone());
        check!(pref.name() == &Some(name));
        check!(pref.goal() == &gd);
    }

    #[test]
    fn new_without_name() {
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::new(None, gd.clone());
        check!(pref.name() == &None);
        check!(pref.goal() == &gd);
    }

    #[test]
    fn from_tuple_with_option() {
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::from((None::<PreferenceName>, gd.clone()));
        check!(pref.name() == &None);
        check!(pref.goal() == &gd);
    }

    #[test]
    fn from_tuple_with_name() {
        let name = PreferenceName::string("my_pref");
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::from((name.clone(), gd.clone()));
        check!(pref.name() == &Some(name));
        check!(pref.goal() == &gd);
    }

    #[test]
    fn from_gd_only() {
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::from(gd.clone());
        check!(pref.name() == &None);
        check!(pref.goal() == &gd);
    }

    #[test]
    fn is_empty_true() {
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::new(None, gd);
        check!(pref.is_empty());
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
        let gd = GoalDescription::AtomicFormula(af);
        let pref = Preference::new(None, gd);
        check!(!pref.is_empty());
    }

    #[test]
    fn clone_works() {
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::new(None, gd);
        let clone = pref.clone();
        check!(pref == clone);
    }

    #[test]
    fn debug_impl() {
        let gd = GoalDescription::and(Vec::new());
        let pref = Preference::new(None, gd);
        let dbg = format!("{pref:?}");
        check!(dbg.contains("Preference"));
    }
}
