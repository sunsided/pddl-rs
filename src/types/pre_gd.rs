//! Contains precondition goal definitions.

use crate::types::TypedVariables;
use crate::types::{Preference, PreferenceGD};

/// A precondition goal definition.
///
/// ## Usage
/// Used by [`PreGD`] itself, as well as [`ActionDefinition`](crate::ActionDefinition).
#[derive(Debug, Clone, PartialEq)]
// TODO: Rename to PreconditionGoalDefinition
pub enum PreGD<'a> {
    // TODO: Unify with base type; should always be a vector; count can be zero.
    And(Vec<PreGD<'a>>),
    /// ## Requirements
    /// None per se: this branch may expand into [`PreferenceGD::Goal`](PreferenceGD::Goal),
    /// which has no requirements.
    Preference(PreferenceGD<'a>),
    /// ## Requirements
    /// Requires [UniversalPreconditions](crate::types::Requirement::UniversalPreconditions).
    Forall(TypedVariables<'a>, Box<PreGD<'a>>),
}

impl<'a> PreGD<'a> {
    pub fn new_preference(pref: PreferenceGD<'a>) -> Self {
        Self::Preference(pref)
    }
    pub fn new_and<I: IntoIterator<Item = PreGD<'a>>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }
    pub fn new_forall(variables: TypedVariables<'a>, gd: PreGD<'a>) -> Self {
        Self::Forall(variables, Box::new(gd))
    }
}

impl<'a> From<PreferenceGD<'a>> for PreGD<'a> {
    fn from(value: PreferenceGD<'a>) -> Self {
        PreGD::new_preference(value)
    }
}

impl<'a> From<Preference<'a>> for PreGD<'a> {
    fn from(value: Preference<'a>) -> Self {
        PreGD::new_preference(PreferenceGD::from_preference(value))
    }
}
