//! Contains the [`DurativeActionEffect`] type.

use crate::types::TypedVariables;
use crate::types::{DurativeActionGoalDefinition, TimedEffect};

/// A durative action effect.
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionEffect<'a> {
    Timed(TimedEffect<'a>),
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<DurativeActionEffect<'a>>),
    /// Requires [ConditionalEffects](crate::types::Requirement::ConditionalEffects).
    Forall(TypedVariables<'a>, Box<DurativeActionEffect<'a>>),
    /// Requires [ConditionalEffects](crate::types::Requirement::ConditionalEffects).
    When(DurativeActionGoalDefinition<'a>, TimedEffect<'a>),
}

impl<'a> DurativeActionEffect<'a> {
    pub const fn new_timed(effect: TimedEffect<'a>) -> Self {
        Self::Timed(effect)
    }
    pub fn new_and<E: IntoIterator<Item = DurativeActionEffect<'a>>>(effect: E) -> Self {
        Self::All(effect.into_iter().collect())
    }
    pub fn new_forall(variables: TypedVariables<'a>, effect: DurativeActionEffect<'a>) -> Self {
        Self::Forall(variables, Box::new(effect))
    }
    pub const fn new_when(gd: DurativeActionGoalDefinition<'a>, effect: TimedEffect<'a>) -> Self {
        Self::When(gd, effect)
    }
}

impl<'a> From<TimedEffect<'a>> for DurativeActionEffect<'a> {
    fn from(value: TimedEffect<'a>) -> Self {
        Self::new_timed(value)
    }
}

impl<'a> FromIterator<DurativeActionEffect<'a>> for DurativeActionEffect<'a> {
    fn from_iter<T: IntoIterator<Item = DurativeActionEffect<'a>>>(iter: T) -> Self {
        Self::new_and(iter)
    }
}

impl<'a> From<(TypedVariables<'a>, DurativeActionEffect<'a>)> for DurativeActionEffect<'a> {
    fn from(value: (TypedVariables<'a>, DurativeActionEffect<'a>)) -> Self {
        Self::new_forall(value.0, value.1)
    }
}

impl<'a> From<(DurativeActionGoalDefinition<'a>, TimedEffect<'a>)> for DurativeActionEffect<'a> {
    fn from(value: (DurativeActionGoalDefinition<'a>, TimedEffect<'a>)) -> Self {
        Self::new_when(value.0, value.1)
    }
}
