//! Contains durative action goal definitions via the [`DurativeActionGoalDefinition`] type.

use crate::types::PrefTimedGD;
use crate::types::TypedVariables;

/// A durative action goal definition.
///
/// ## Usage
/// Used by [`DurativeActionGoalDefinition`] itself, as well as [`DurativeActionDefinition`](crate::DurativeActionDefinition) and
/// [`DurativeActionEffect`](crate::DurativeActionEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionGoalDefinition {
    Timed(PrefTimedGD),
    And(Vec<DurativeActionGoalDefinition>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables, Box<DurativeActionGoalDefinition>),
}

impl DurativeActionGoalDefinition {
    pub fn new_timed(pref: PrefTimedGD) -> Self {
        Self::Timed(pref)
    }
    pub fn new_and<I: IntoIterator<Item = DurativeActionGoalDefinition>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }
    pub fn new_forall(variables: TypedVariables, gd: DurativeActionGoalDefinition) -> Self {
        Self::Forall(variables, Box::new(gd))
    }
}

impl From<PrefTimedGD> for DurativeActionGoalDefinition {
    fn from(value: PrefTimedGD) -> Self {
        DurativeActionGoalDefinition::new_timed(value)
    }
}
