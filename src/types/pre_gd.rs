//! Contains precondition goal definitions.

use crate::types::TypedList;
use crate::types::{Preference, PreferenceGD, Variable};

/// A precondition goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum PreGD<'a> {
    Preference(PreferenceGD<'a>),
    And(Vec<PreGD<'a>>),
    /// Requires [UniversalPreconditions](crate::types::Requirement::UniversalPreconditions).
    Forall(TypedList<'a, Variable<'a>>, Box<PreGD<'a>>),
}

impl<'a> PreGD<'a> {
    pub fn new_preference(pref: PreferenceGD<'a>) -> Self {
        Self::Preference(pref)
    }
    pub fn new_and<I: IntoIterator<Item = PreGD<'a>>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }
    pub fn new_forall(variables: TypedList<'a, Variable<'a>>, gd: PreGD<'a>) -> Self {
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
