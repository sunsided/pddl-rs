//! Contains precondition goal definitions.

use crate::types::TypedVariables;
use crate::types::{Preference, PreferenceGD};

/// A precondition goal definition.
///
/// ## Usage
/// Used by [`PreconditionGoalDefinition`] itself, as well as [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum PreconditionGoalDefinition<'a> {
    // TODO: Unify with base type; should always be a vector; count can be zero.
    And(Vec<PreconditionGoalDefinition<'a>>),
    /// ## Requirements
    /// None per se: this branch may expand into [`PreferenceGD::Goal`](PreferenceGD::Goal),
    /// which has no requirements.
    Preference(PreferenceGD<'a>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    Forall(TypedVariables<'a>, Box<PreconditionGoalDefinition<'a>>),
}

impl<'a> PreconditionGoalDefinition<'a> {
    pub fn new_preference(pref: PreferenceGD<'a>) -> Self {
        Self::Preference(pref)
    }
    pub fn new_and<I: IntoIterator<Item = PreconditionGoalDefinition<'a>>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }
    pub fn new_forall(variables: TypedVariables<'a>, gd: PreconditionGoalDefinition<'a>) -> Self {
        Self::Forall(variables, Box::new(gd))
    }
}

impl<'a> From<PreferenceGD<'a>> for PreconditionGoalDefinition<'a> {
    fn from(value: PreferenceGD<'a>) -> Self {
        PreconditionGoalDefinition::new_preference(value)
    }
}

impl<'a> From<Preference<'a>> for PreconditionGoalDefinition<'a> {
    fn from(value: Preference<'a>) -> Self {
        PreconditionGoalDefinition::new_preference(PreferenceGD::from_preference(value))
    }
}
