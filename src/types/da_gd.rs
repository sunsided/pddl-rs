//! Contains durative action goal definitions via the [`DurativeActionGoalDefinition`] type.

use crate::types::PreferenceTimedGoalDefinition;
use crate::types::TypedVariables;

/// A durative action goal definition.
///
/// ## Usage
/// Used by [`DurativeActionGoalDefinition`] itself, as well as [`DurativeActionDefinition`](crate::DurativeActionDefinition) and
/// [`DurativeActionEffect`](crate::DurativeActionEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionGoalDefinition {
    /// A (preferred) timed goal, `<pref-timed-GD>`.
    Timed(PreferenceTimedGoalDefinition),
    /// A conjunction, `(and <da-GD>*)`.
    And(Vec<DurativeActionGoalDefinition>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, Box<DurativeActionGoalDefinition>),
}

impl DurativeActionGoalDefinition {
    /// Creates a timed goal from a [`PreferenceTimedGoalDefinition`].
    #[doc(alias = "new_timed")]
    pub fn timed(pref: PreferenceTimedGoalDefinition) -> Self {
        Self::Timed(pref)
    }

    /// Alias for [`Self::timed`].
    pub fn new_timed(pref: PreferenceTimedGoalDefinition) -> Self {
        Self::timed(pref)
    }

    /// Creates a conjunction `(and <da-GD>*)`.
    #[doc(alias = "new_and")]
    pub fn and<I: IntoIterator<Item = DurativeActionGoalDefinition>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }

    /// Alias for [`Self::and`].
    pub fn new_and<I: IntoIterator<Item = DurativeActionGoalDefinition>>(prefs: I) -> Self {
        Self::and(prefs)
    }

    /// Creates a universal quantification `(forall (<variables>) <da-GD>)`.
    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: DurativeActionGoalDefinition) -> Self {
        Self::Forall(variables, Box::new(gd))
    }

    /// Alias for [`Self::forall`].
    pub fn new_forall(variables: TypedVariables, gd: DurativeActionGoalDefinition) -> Self {
        Self::r#forall(variables, gd)
    }
}

impl From<PreferenceTimedGoalDefinition> for DurativeActionGoalDefinition {
    fn from(value: PreferenceTimedGoalDefinition) -> Self {
        DurativeActionGoalDefinition::timed(value)
    }
}
