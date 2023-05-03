//! Contains the [`CEffect`] type.

use crate::types::TypedVariables;
use crate::types::{ConditionalEffect, Effect, GoalDefinition, PEffect};

/// A conditional effect. Occurs as part of [`Effect`].
///
/// ## Usage
/// Used by [`Effect`](Effect).
#[derive(Debug, Clone, PartialEq)]
pub enum CEffect<'a> {
    Effect(PEffect<'a>),
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    Forall(TypedVariables<'a>, Box<Effect<'a>>),
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    When(GoalDefinition<'a>, ConditionalEffect<'a>),
}

impl<'a> CEffect<'a> {
    pub const fn new_p_effect(effect: PEffect<'a>) -> Self {
        Self::Effect(effect)
    }

    pub fn new_forall(variables: TypedVariables<'a>, effect: Effect<'a>) -> Self {
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

impl<'a> From<(TypedVariables<'a>, Effect<'a>)> for CEffect<'a> {
    fn from(value: (TypedVariables<'a>, Effect<'a>)) -> Self {
        CEffect::new_forall(value.0, value.1)
    }
}

impl<'a> From<(GoalDefinition<'a>, ConditionalEffect<'a>)> for CEffect<'a> {
    fn from(value: (GoalDefinition<'a>, ConditionalEffect<'a>)) -> Self {
        CEffect::new_when(value.0, value.1)
    }
}
