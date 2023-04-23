//! Contains c-effects.

use crate::types::domain::{
    ConditionalEffect, Effect, GoalDefinition, PEffect, TypedList, Variable,
};

/// A c-effect.
#[derive(Debug, Clone, PartialEq)]
pub enum CEffect<'a> {
    PEffect(PEffect<'a>),
    /// Requires [ConditionalEffects](crate::types::Requirement::ConditionalEffects).
    Forall(TypedList<'a, Variable<'a>>, Box<Effect<'a>>),
    /// Requires [ConditionalEffects](crate::types::Requirement::ConditionalEffects).
    When(GoalDefinition<'a>, ConditionalEffect<'a>),
}

impl<'a> CEffect<'a> {
    pub const fn new_p_effect(effect: PEffect<'a>) -> Self {
        Self::PEffect(effect)
    }

    pub fn new_forall(variables: TypedList<'a, Variable<'a>>, effect: Effect<'a>) -> Self {
        Self::Forall(variables, Box::new(effect))
    }

    pub const fn new_when(gd: GoalDefinition<'a>, effect: ConditionalEffect<'a>) -> Self {
        Self::When(gd, effect)
    }
}

impl<'a> From<PEffect<'a>> for CEffect<'a> {
    fn from(value: PEffect<'a>) -> Self {
        CEffect::new_p_effect(value)
    }
}

impl<'a> From<(TypedList<'a, Variable<'a>>, Effect<'a>)> for CEffect<'a> {
    fn from(value: (TypedList<'a, Variable<'a>>, Effect<'a>)) -> Self {
        CEffect::new_forall(value.0, value.1)
    }
}

impl<'a> From<(GoalDefinition<'a>, ConditionalEffect<'a>)> for CEffect<'a> {
    fn from(value: (GoalDefinition<'a>, ConditionalEffect<'a>)) -> Self {
        CEffect::new_when(value.0, value.1)
    }
}
