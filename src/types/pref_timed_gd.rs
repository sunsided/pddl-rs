//! Contains the [`PreferenceTimedGoalDefinition`] type.

use crate::types::{PreferenceName, TimedGoalDefinition};

/// A (preferred) timed goal definition.
///
/// ## Usage
/// Used by [`DurativeActionGoalDefinition`](crate::DurativeActionGoalDefinition).
#[doc(alias("pref-timed-GD"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceTimedGoalDefinition {
    Required(TimedGoalDefinition),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName>, TimedGoalDefinition),
}

impl PreferenceTimedGoalDefinition {
    #[doc(alias = "new_required")]
    pub const fn required(gd: TimedGoalDefinition) -> Self {
        Self::Required(gd)
    }

    #[doc(alias = "new_preference")]
    pub const fn preference(name: Option<PreferenceName>, gd: TimedGoalDefinition) -> Self {
        Self::Preference(name, gd)
    }

    #[deprecated(since = "0.2.0", note = "Use `required` instead")]
    pub const fn new_required(gd: TimedGoalDefinition) -> Self {
        Self::required(gd)
    }

    #[deprecated(since = "0.2.0", note = "Use `preference` instead")]
    pub const fn new_preference(name: Option<PreferenceName>, gd: TimedGoalDefinition) -> Self {
        Self::preference(name, gd)
    }
}

impl From<TimedGoalDefinition> for PreferenceTimedGoalDefinition {
    fn from(value: TimedGoalDefinition) -> Self {
        PreferenceTimedGoalDefinition::Required(value)
    }
}

impl From<(Option<PreferenceName>, TimedGoalDefinition)> for PreferenceTimedGoalDefinition {
    fn from(value: (Option<PreferenceName>, TimedGoalDefinition)) -> Self {
        PreferenceTimedGoalDefinition::Preference(value.0, value.1)
    }
}

/// Alias for [`PreferenceTimedGoalDefinition`]; matches BNF `<pref-timed-GD>`.
#[deprecated(since = "0.2.0", note = "Use `PreferenceTimedGoalDefinition` instead")]
pub type PrefTimedGD = PreferenceTimedGoalDefinition;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AtomicFormula, GoalDefinition, Predicate, Term, TimeSpecifier, TimedGoalDefinition,
    };

    fn make_timed_gd() -> TimedGoalDefinition {
        let af = AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new());
        let gd = GoalDefinition::AtomicFormula(af);
        TimedGoalDefinition::at(TimeSpecifier::Start, gd)
    }

    #[test]
    fn required_constructor() {
        let timed_gd = make_timed_gd();
        let pref = PreferenceTimedGoalDefinition::required(timed_gd.clone());
        assert_eq!(pref, PreferenceTimedGoalDefinition::Required(timed_gd));
    }

    #[test]
    fn preference_with_name() {
        let timed_gd = make_timed_gd();
        let name = PreferenceName::new("pref1");
        let pref = PreferenceTimedGoalDefinition::preference(Some(name.clone()), timed_gd.clone());
        assert_eq!(
            pref,
            PreferenceTimedGoalDefinition::Preference(Some(name), timed_gd)
        );
    }

    #[test]
    fn preference_without_name() {
        let timed_gd = make_timed_gd();
        let pref = PreferenceTimedGoalDefinition::preference(None, timed_gd.clone());
        assert_eq!(
            pref,
            PreferenceTimedGoalDefinition::Preference(None, timed_gd)
        );
    }

    #[test]
    fn from_timed_gd() {
        let timed_gd = make_timed_gd();
        let from_gd: PreferenceTimedGoalDefinition = timed_gd.clone().into();
        assert_eq!(from_gd, PreferenceTimedGoalDefinition::Required(timed_gd));
    }

    #[test]
    fn from_tuple() {
        let timed_gd = make_timed_gd();
        let name = PreferenceName::new("pref1");
        let from_tuple: PreferenceTimedGoalDefinition =
            (Some(name.clone()), timed_gd.clone()).into();
        assert_eq!(
            from_tuple,
            PreferenceTimedGoalDefinition::Preference(Some(name), timed_gd)
        );
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let timed_gd = make_timed_gd();
        let _ = PreferenceTimedGoalDefinition::new_required(timed_gd.clone());
        let _ =
            PreferenceTimedGoalDefinition::new_preference(Some(PreferenceName::new("p")), timed_gd);
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias() {
        fn _assert_alias(_: PrefTimedGD) {}
    }
}
