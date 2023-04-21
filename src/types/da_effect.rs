//! Contains the [`DAEffect`] type.

use crate::types::{TimedEffect, TypedList, Variable, DAGD};

/// A durative action effect.
#[derive(Debug, Clone, PartialEq)]
pub enum DAEffect<'a> {
    Timed(TimedEffect<'a>),
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<DAEffect<'a>>),
    /// Requires [ConditionalEffects](crate::types::Requirement::ConditionalEffects).
    Forall(TypedList<'a, Variable<'a>>, Box<DAEffect<'a>>),
    /// Requires [ConditionalEffects](crate::types::Requirement::ConditionalEffects).
    When(DAGD<'a>, TimedEffect<'a>),
}

impl<'a> DAEffect<'a> {
    pub const fn new_timed(effect: TimedEffect<'a>) -> Self {
        Self::Timed(effect)
    }
    pub fn new_and<E: IntoIterator<Item = DAEffect<'a>>>(effect: E) -> Self {
        Self::All(effect.into_iter().collect())
    }
    pub fn new_forall(variables: TypedList<'a, Variable<'a>>, effect: DAEffect<'a>) -> Self {
        Self::Forall(variables, Box::new(effect))
    }
    pub const fn new_when(gd: DAGD<'a>, effect: TimedEffect<'a>) -> Self {
        Self::When(gd, effect)
    }
}

impl<'a> From<TimedEffect<'a>> for DAEffect<'a> {
    fn from(value: TimedEffect<'a>) -> Self {
        Self::new_timed(value)
    }
}

impl<'a> FromIterator<DAEffect<'a>> for DAEffect<'a> {
    fn from_iter<T: IntoIterator<Item = DAEffect<'a>>>(iter: T) -> Self {
        Self::new_and(iter)
    }
}

impl<'a> From<(TypedList<'a, Variable<'a>>, DAEffect<'a>)> for DAEffect<'a> {
    fn from(value: (TypedList<'a, Variable<'a>>, DAEffect<'a>)) -> Self {
        Self::new_forall(value.0, value.1)
    }
}

impl<'a> From<(DAGD<'a>, TimedEffect<'a>)> for DAEffect<'a> {
    fn from(value: (DAGD<'a>, TimedEffect<'a>)) -> Self {
        Self::new_when(value.0, value.1)
    }
}
