//! Contains the [`PreferenceTimedGoalDescription`] type.

use crate::types::{PreferenceName, TimedGoalDescription};

/// A timed goal description that may be a preference, `<pref-timed-GD>`.
///
/// ## Usage
/// Used by [`DurativeActionGoalDescription`](crate::DurativeActionGoalDescription).
#[doc(alias("pref-timed-GD"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PreferenceTimedGoalDescription {
    /// A required timed goal, `<timed-GD>`.
    Required(TimedGoalDescription),
    /// ## Requirements
    /// Requires [Preferences](crate::Requirement::Preferences).
    Preference(Option<PreferenceName>, TimedGoalDescription),
}

impl PreferenceTimedGoalDescription {
    /// Creates a required timed goal.
    #[doc(alias = "new_required")]
    #[must_use]
    pub const fn required(gd: TimedGoalDescription) -> Self {
        Self::Required(gd)
    }

    /// Creates a preference `(preference [<pref-name>] <timed-GD>)` with an optional name.
    #[doc(alias = "new_preference")]
    #[must_use]
    pub const fn preference(name: Option<PreferenceName>, gd: TimedGoalDescription) -> Self {
        Self::Preference(name, gd)
    }

    /// Deprecated alias for [`Self::required`].
    #[deprecated(since = "0.2.0", note = "Use `required` instead")]
    #[must_use]
    pub const fn new_required(gd: TimedGoalDescription) -> Self {
        Self::required(gd)
    }

    /// Deprecated alias for [`Self::preference`].
    #[deprecated(since = "0.2.0", note = "Use `preference` instead")]
    #[must_use]
    pub const fn new_preference(name: Option<PreferenceName>, gd: TimedGoalDescription) -> Self {
        Self::preference(name, gd)
    }
}

impl From<TimedGoalDescription> for PreferenceTimedGoalDescription {
    fn from(value: TimedGoalDescription) -> Self {
        PreferenceTimedGoalDescription::Required(value)
    }
}

impl From<(Option<PreferenceName>, TimedGoalDescription)> for PreferenceTimedGoalDescription {
    fn from(value: (Option<PreferenceName>, TimedGoalDescription)) -> Self {
        PreferenceTimedGoalDescription::Preference(value.0, value.1)
    }
}

/// Alias for [`PreferenceTimedGoalDescription`]; matches BNF `<pref-timed-GD>`.
#[deprecated(since = "0.2.0", note = "Use `PreferenceTimedGoalDescription` instead")]
pub type PrefTimedGD = PreferenceTimedGoalDescription;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AtomicFormula, GoalDescription, Predicate, Term, TimeSpecifier, TimedGoalDescription,
    };
    use assert2::check;

    fn make_timed_gd() -> TimedGoalDescription {
        let af = AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new());
        let gd = GoalDescription::AtomicFormula(af);
        TimedGoalDescription::at(TimeSpecifier::Start, gd)
    }

    #[test]
    fn required_constructor() {
        let timed_gd = make_timed_gd();
        let pref = PreferenceTimedGoalDescription::required(timed_gd.clone());
        check!(pref == PreferenceTimedGoalDescription::Required(timed_gd));
    }

    #[test]
    fn preference_with_name() {
        let timed_gd = make_timed_gd();
        let name = PreferenceName::new("pref1");
        let pref = PreferenceTimedGoalDescription::preference(Some(name.clone()), timed_gd.clone());
        check!(pref == PreferenceTimedGoalDescription::Preference(Some(name), timed_gd));
    }

    #[test]
    fn preference_without_name() {
        let timed_gd = make_timed_gd();
        let pref = PreferenceTimedGoalDescription::preference(None, timed_gd.clone());
        check!(pref == PreferenceTimedGoalDescription::Preference(None, timed_gd));
    }

    #[test]
    fn from_timed_gd() {
        let timed_gd = make_timed_gd();
        let from_gd: PreferenceTimedGoalDescription = timed_gd.clone().into();
        check!(from_gd == PreferenceTimedGoalDescription::Required(timed_gd));
    }

    #[test]
    fn from_tuple() {
        let timed_gd = make_timed_gd();
        let name = PreferenceName::new("pref1");
        let from_tuple: PreferenceTimedGoalDescription =
            (Some(name.clone()), timed_gd.clone()).into();
        check!(from_tuple == PreferenceTimedGoalDescription::Preference(Some(name), timed_gd));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let timed_gd = make_timed_gd();
        let _ = PreferenceTimedGoalDescription::new_required(timed_gd.clone());
        let _ = PreferenceTimedGoalDescription::new_preference(
            Some(PreferenceName::new("p")),
            timed_gd,
        );
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias() {
        fn _assert_alias(_: PrefTimedGD) {}
    }
}
