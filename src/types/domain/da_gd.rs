//! Contains the [`DurativeActionGoalDefinition`] type.

use crate::types::domain::{PrefTimedGD, TypedList, Variable};

/// A durative action goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionGoalDefinition<'a> {
    Timed(PrefTimedGD<'a>),
    And(Vec<DurativeActionGoalDefinition<'a>>),
    /// Requires [UniversalPreconditions](crate::types::Requirement::UniversalPreconditions).
    Forall(
        TypedList<'a, Variable<'a>>,
        Box<DurativeActionGoalDefinition<'a>>,
    ),
}

impl<'a> DurativeActionGoalDefinition<'a> {
    pub fn new_timed(pref: PrefTimedGD<'a>) -> Self {
        Self::Timed(pref)
    }
    pub fn new_and<I: IntoIterator<Item = DurativeActionGoalDefinition<'a>>>(prefs: I) -> Self {
        Self::And(prefs.into_iter().collect())
    }
    pub fn new_forall(
        variables: TypedList<'a, Variable<'a>>,
        gd: DurativeActionGoalDefinition<'a>,
    ) -> Self {
        Self::Forall(variables, Box::new(gd))
    }
}

impl<'a> From<PrefTimedGD<'a>> for DurativeActionGoalDefinition<'a> {
    fn from(value: PrefTimedGD<'a>) -> Self {
        DurativeActionGoalDefinition::new_timed(value)
    }
}
