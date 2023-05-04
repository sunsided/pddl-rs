//! Contains the [`CEffect`] type.

use crate::types::TypedVariables;
use crate::types::{ConditionalEffect, Effects, GoalDefinition, PEffect};

/// A (potentially conditional) effect. Occurs as part of [`Effects`].
///
/// ## Usage
/// Used by [`Effect`](Effects).
#[derive(Debug, Clone, PartialEq)]
pub enum CEffect<'a> {
    /// A regular effect.
    Effect(PEffect<'a>),
    /// A universal affect that is conditioned by variables.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    Forall(ForallCEffect<'a>),
    /// A conditional effect that applies only if a given
    /// precondition holds true.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    When(WhenCEffect<'a>),
}

/// Applies the specified effects for all listed variables.
///
/// ## Requirements
/// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
#[derive(Debug, Clone, PartialEq)]
pub struct ForallCEffect<'a> {
    /// The (possibly empty) set of variables constraining the effects.
    pub variables: TypedVariables<'a>,
    /// The effects to apply.
    pub effects: Effects<'a>,
}

impl<'a> ForallCEffect<'a> {
    /// Creates a new [`ForallCEffect`] value.
    pub const fn new(variables: TypedVariables<'a>, effects: Effects<'a>) -> Self {
        ForallCEffect { variables, effects }
    }
}

/// Applies the specified effects for all listed variables.
///
/// ## Requirements
/// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
#[derive(Debug, Clone, PartialEq)]
pub struct WhenCEffect<'a> {
    /// The antecedent. This needs to be true for the effect to apply.
    pub condition: GoalDefinition<'a>,
    /// The consequent. This is the effect that applies when the condition is true.
    pub effect: ConditionalEffect<'a>,
}

impl<'a> WhenCEffect<'a> {
    /// Creates a new [`WhenCEffect`] value.
    pub const fn new(condition: GoalDefinition<'a>, effect: ConditionalEffect<'a>) -> Self {
        WhenCEffect { condition, effect }
    }
}

impl<'a> CEffect<'a> {
    /// Crates a new effect.
    pub const fn new_p_effect(effect: PEffect<'a>) -> Self {
        Self::Effect(effect)
    }

    /// Creates a new universal effect that applies to all
    /// the specified variables.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    pub const fn new_forall(variables: TypedVariables<'a>, effect: Effects<'a>) -> Self {
        Self::Forall(ForallCEffect::new(variables, effect))
    }

    /// Creates a new conditional effect.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    pub const fn new_when(condition: GoalDefinition<'a>, effect: ConditionalEffect<'a>) -> Self {
        Self::When(WhenCEffect::new(condition, effect))
    }
}

impl<'a> From<PEffect<'a>> for CEffect<'a> {
    fn from(value: PEffect<'a>) -> Self {
        CEffect::new_p_effect(value)
    }
}

impl<'a> From<ForallCEffect<'a>> for CEffect<'a> {
    fn from(value: ForallCEffect<'a>) -> Self {
        CEffect::Forall(value)
    }
}

impl<'a> From<WhenCEffect<'a>> for CEffect<'a> {
    fn from(value: WhenCEffect<'a>) -> Self {
        CEffect::When(value)
    }
}

impl<'a> From<(TypedVariables<'a>, Effects<'a>)> for CEffect<'a> {
    fn from(value: (TypedVariables<'a>, Effects<'a>)) -> Self {
        CEffect::new_forall(value.0, value.1)
    }
}

impl<'a> From<(GoalDefinition<'a>, ConditionalEffect<'a>)> for CEffect<'a> {
    fn from(value: (GoalDefinition<'a>, ConditionalEffect<'a>)) -> Self {
        CEffect::new_when(value.0, value.1)
    }
}

impl<'a> TryInto<PEffect<'a>> for CEffect<'a> {
    type Error = ();

    fn try_into(self) -> Result<PEffect<'a>, Self::Error> {
        match self {
            CEffect::Effect(x) => Ok(x),
            CEffect::Forall(_) => Err(()),
            CEffect::When(_) => Err(()),
        }
    }
}

impl<'a> TryInto<WhenCEffect<'a>> for CEffect<'a> {
    type Error = ();

    fn try_into(self) -> Result<WhenCEffect<'a>, Self::Error> {
        match self {
            CEffect::Effect(_) => Err(()),
            CEffect::Forall(_) => Err(()),
            CEffect::When(x) => Ok(x),
        }
    }
}

impl<'a> TryInto<ForallCEffect<'a>> for CEffect<'a> {
    type Error = ();

    fn try_into(self) -> Result<ForallCEffect<'a>, Self::Error> {
        match self {
            CEffect::Effect(_) => Err(()),
            CEffect::Forall(x) => Ok(x),
            CEffect::When(_) => Err(()),
        }
    }
}
