//! Contains durative action goal descriptions via the [`DurativeActionGoalDescription`] type.

use crate::types::PreferenceTimedGoalDescription;
use crate::types::TypedVariables;

/// A durative action goal description.
///
/// ## Usage
/// Used by [`DurativeActionGoalDescription`] itself, as well as [`DurativeActionDefinition`](crate::DurativeActionDefinition) and
/// [`DurativeActionEffect`](crate::DurativeActionEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionGoalDescription {
    /// A (preferred) timed goal, `<pref-timed-GD>`.
    Timed(PreferenceTimedGoalDescription),
    /// A conjunction, `(and <da-GD>*)`.
    And(Vec<DurativeActionGoalDescription>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, Box<DurativeActionGoalDescription>),
}

impl DurativeActionGoalDescription {
    /// Creates a timed goal from a [`PreferenceTimedGoalDescription`].
    #[doc(alias = "new_timed")]
    pub fn timed(pref: PreferenceTimedGoalDescription) -> Self {
        Self::Timed(pref)
    }

    /// Alias for [`Self::timed`].
    pub fn new_timed(pref: PreferenceTimedGoalDescription) -> Self {
        Self::timed(pref)
    }

    /// Creates a conjunction `(and <da-GD>*)`.
    #[doc(alias = "new_and")]
    pub fn and<I: IntoIterator<Item = DurativeActionGoalDescription>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }

    /// Alias for [`Self::and`].
    pub fn new_and<I: IntoIterator<Item = DurativeActionGoalDescription>>(prefs: I) -> Self {
        Self::and(prefs)
    }

    /// Creates a universal quantification `(forall (<variables>) <da-GD>)`.
    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: DurativeActionGoalDescription) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    /// Alias for [`Self::forall`].
    pub fn new_forall(variables: TypedVariables, gd: DurativeActionGoalDescription) -> Self {
        Self::r#forall(variables, gd)
    }
}

impl From<PreferenceTimedGoalDescription> for DurativeActionGoalDescription {
    fn from(value: PreferenceTimedGoalDescription) -> Self {
        DurativeActionGoalDescription::timed(value)
    }
}
