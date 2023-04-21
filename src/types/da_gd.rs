//! Contains the [`DAGD`] type.

use crate::types::{PrefTimedGD, TypedList, Variable};

/// A durative action goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum DAGD<'a> {
    Timed(PrefTimedGD<'a>),
    And(Vec<DAGD<'a>>),
    /// Requires [UniversalPreconditions](crate::types::Requirement::UniversalPreconditions).
    Forall(TypedList<'a, Variable<'a>>, Box<DAGD<'a>>),
}

impl<'a> DAGD<'a> {
    pub fn new_timed(pref: PrefTimedGD<'a>) -> Self {
        Self::Timed(pref)
    }
    pub fn new_and<I: IntoIterator<Item = DAGD<'a>>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }
    pub fn new_forall(variables: TypedList<'a, Variable<'a>>, gd: DAGD<'a>) -> Self {
        Self::Forall(variables, Box::new(gd))
    }
}

impl<'a> From<PrefTimedGD<'a>> for DAGD<'a> {
    fn from(value: PrefTimedGD<'a>) -> Self {
        DAGD::new_timed(value)
    }
}
