//! Contains durative action goal definitions via the [`DurativeActionGoalDefinition`] type.

use crate::types::PrefTimedGD;
use crate::types::TypedVariables;

/// A durative action goal definition.
///
/// ## Usage
/// Used by [`DurativeActionGoalDefinition`] itself, as well as [`DurativeActionDefinition`](crate::DurativeActionDefinition) and
/// [`DurativeActionEffect`](crate::DurativeActionEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionGoalDefinition<'a> {
    Timed(PrefTimedGD<'a>),
    And(Vec<DurativeActionGoalDefinition<'a>>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables<'a>, Box<DurativeActionGoalDefinition<'a>>),
}

impl<'a> DurativeActionGoalDefinition<'a> {
    pub fn new_timed(pref: PrefTimedGD<'a>) -> Self {
        Self::Timed(pref)
    }
    pub fn new_and<I: IntoIterator<Item = DurativeActionGoalDefinition<'a>>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }
    pub fn new_forall(variables: TypedVariables<'a>, gd: DurativeActionGoalDefinition<'a>) -> Self {
        Self::Forall(variables, Box::new(gd))
    }
}

impl<'a> From<PrefTimedGD<'a>> for DurativeActionGoalDefinition<'a> {
    fn from(value: PrefTimedGD<'a>) -> Self {
        DurativeActionGoalDefinition::new_timed(value)
    }
}
