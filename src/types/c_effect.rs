//! Contains the [`CEffect`] type.

use crate::types::TypedVariables;
use crate::types::{ConditionalEffect, Effects, GoalDefinition, PEffect};

/// A (potentially conditional) effect. Occurs as part of [`Effects`].
///
/// ## Usage
/// Used by [`Effect`](Effects).
#[derive(Debug, Clone, PartialEq)]
pub enum CEffect {
    /// A regular effect.
    Effect(PEffect),
    /// A universal affect that is conditioned by variables.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    Forall(ForallCEffect),
    /// A conditional effect that applies only if a given
    /// precondition holds true.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    When(WhenCEffect),
}

/// Applies the specified effects for all listed variables.
///
/// ## Requirements
/// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
#[derive(Debug, Clone, PartialEq)]
pub struct ForallCEffect {
    /// The (possibly empty) set of variables constraining the effects.
    pub variables: TypedVariables,
    /// The effects to apply.
    pub effects: Effects,
}

impl ForallCEffect {
    /// Creates a new [`ForallCEffect`] value.
    pub const fn new(variables: TypedVariables, effects: Effects) -> Self {
        ForallCEffect { variables, effects }
    }
}

/// Applies the specified effects for all listed variables.
///
/// ## Requirements
/// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
#[derive(Debug, Clone, PartialEq)]
pub struct WhenCEffect {
    /// The antecedent. This needs to be true for the effect to apply.
    pub condition: GoalDefinition,
    /// The consequent. This is the effect that applies when the condition is true.
    pub effect: ConditionalEffect,
}

impl WhenCEffect {
    /// Creates a new [`WhenCEffect`] value.
    pub const fn new(condition: GoalDefinition, effect: ConditionalEffect) -> Self {
        WhenCEffect { condition, effect }
    }
}

impl CEffect {
    /// Crates a new effect.
    pub const fn new_p_effect(effect: PEffect) -> Self {
        Self::Effect(effect)
    }

    /// Creates a new universal effect that applies to all
    /// the specified variables.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    pub const fn new_forall(variables: TypedVariables, effect: Effects) -> Self {
        Self::Forall(ForallCEffect::new(variables, effect))
    }

    /// Creates a new conditional effect.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    pub const fn new_when(condition: GoalDefinition, effect: ConditionalEffect) -> Self {
        Self::When(WhenCEffect::new(condition, effect))
    }
}

impl From<PEffect> for CEffect {
    fn from(value: PEffect) -> Self {
        CEffect::new_p_effect(value)
    }
}

impl From<ForallCEffect> for CEffect {
    fn from(value: ForallCEffect) -> Self {
        CEffect::Forall(value)
    }
}

impl From<WhenCEffect> for CEffect {
    fn from(value: WhenCEffect) -> Self {
        CEffect::When(value)
    }
}

impl From<(TypedVariables, Effects)> for ForallCEffect {
    fn from(value: (TypedVariables, Effects)) -> Self {
        ForallCEffect::new(value.0, value.1)
    }
}

impl From<(GoalDefinition, ConditionalEffect)> for WhenCEffect {
    fn from(value: (GoalDefinition, ConditionalEffect)) -> Self {
        WhenCEffect::new(value.0, value.1)
    }
}

impl TryInto<PEffect> for CEffect {
    type Error = ();

    fn try_into(self) -> Result<PEffect, Self::Error> {
        match self {
            CEffect::Effect(x) => Ok(x),
            CEffect::Forall(_) => Err(()),
            CEffect::When(_) => Err(()),
        }
    }
}

impl TryInto<WhenCEffect> for CEffect {
    type Error = ();

    fn try_into(self) -> Result<WhenCEffect, Self::Error> {
        match self {
            CEffect::Effect(_) => Err(()),
            CEffect::Forall(_) => Err(()),
            CEffect::When(x) => Ok(x),
        }
    }
}

impl TryInto<ForallCEffect> for CEffect {
    type Error = ();

    fn try_into(self) -> Result<ForallCEffect, Self::Error> {
        match self {
            CEffect::Effect(_) => Err(()),
            CEffect::Forall(x) => Ok(x),
            CEffect::When(_) => Err(()),
        }
    }
}
