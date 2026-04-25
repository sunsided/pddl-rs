//! Contains the [`ConditionalEffect`] type.

use crate::types::TypedVariables;
use crate::types::{EffectCondition, Effects, GoalDefinition, PrimitiveEffect};

/// A (potentially conditional) effect. Occurs as part of [`Effects`].
///
/// ## Usage
/// Used by [`Effect`](Effects).
#[doc(alias("c-effect"))]
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalEffect {
    /// A regular effect.
    Effect(PrimitiveEffect),
    /// A universal affect that is conditioned by variables.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    Forall(ForallConditionalEffect),
    /// A conditional effect that applies only if a given
    /// precondition holds true.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    When(WhenConditionalEffect),
}

/// Applies the specified effects for all listed variables.
///
/// ## Requirements
/// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
#[doc(alias("c-effect"))]
#[derive(Debug, Clone, PartialEq)]
pub struct ForallConditionalEffect {
    /// The (possibly empty) set of variables constraining the effects.
    pub variables: TypedVariables,
    /// The effects to apply.
    pub effects: Effects,
}

impl ForallConditionalEffect {
    /// Creates a new [`ForallConditionalEffect`] value.
    pub const fn new(variables: TypedVariables, effects: Effects) -> Self {
        ForallConditionalEffect { variables, effects }
    }
}

/// Applies the specified effects for all listed variables.
///
/// ## Requirements
/// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
#[doc(alias("c-effect"))]
#[derive(Debug, Clone, PartialEq)]
pub struct WhenConditionalEffect {
    /// The antecedent. This needs to be true for the effect to apply.
    pub condition: GoalDefinition,
    /// The consequent. This is the effect that applies when the condition is true.
    pub effect: EffectCondition,
}

impl WhenConditionalEffect {
    /// Creates a new [`WhenConditionalEffect`] value.
    pub const fn new(condition: GoalDefinition, effect: EffectCondition) -> Self {
        WhenConditionalEffect { condition, effect }
    }
}

impl ConditionalEffect {
    /// Crates a new effect.
    pub const fn new_primitive_effect(effect: PrimitiveEffect) -> Self {
        Self::Effect(effect)
    }

    /// Creates a new effect.
    #[deprecated(since = "0.2.0", note = "Use `new_primitive_effect` instead")]
    pub const fn new_p_effect(effect: PrimitiveEffect) -> Self {
        Self::new_primitive_effect(effect)
    }

    /// Creates a new universal effect that applies to all
    /// the specified variables.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    pub const fn new_forall(variables: TypedVariables, effect: Effects) -> Self {
        Self::Forall(ForallConditionalEffect::new(variables, effect))
    }

    /// Creates a new conditional effect.
    ///
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    pub const fn new_when(condition: GoalDefinition, effect: EffectCondition) -> Self {
        Self::When(WhenConditionalEffect::new(condition, effect))
    }
}

impl From<PrimitiveEffect> for ConditionalEffect {
    fn from(value: PrimitiveEffect) -> Self {
        ConditionalEffect::new_primitive_effect(value)
    }
}

impl From<ForallConditionalEffect> for ConditionalEffect {
    fn from(value: ForallConditionalEffect) -> Self {
        ConditionalEffect::Forall(value)
    }
}

impl From<WhenConditionalEffect> for ConditionalEffect {
    fn from(value: WhenConditionalEffect) -> Self {
        ConditionalEffect::When(value)
    }
}

impl From<(TypedVariables, Effects)> for ForallConditionalEffect {
    fn from(value: (TypedVariables, Effects)) -> Self {
        ForallConditionalEffect::new(value.0, value.1)
    }
}

impl From<(GoalDefinition, EffectCondition)> for WhenConditionalEffect {
    fn from(value: (GoalDefinition, EffectCondition)) -> Self {
        WhenConditionalEffect::new(value.0, value.1)
    }
}

impl TryInto<PrimitiveEffect> for ConditionalEffect {
    type Error = ();

    fn try_into(self) -> Result<PrimitiveEffect, Self::Error> {
        match self {
            ConditionalEffect::Effect(x) => Ok(x),
            ConditionalEffect::Forall(_) => Err(()),
            ConditionalEffect::When(_) => Err(()),
        }
    }
}

impl TryInto<WhenConditionalEffect> for ConditionalEffect {
    type Error = ();

    fn try_into(self) -> Result<WhenConditionalEffect, Self::Error> {
        match self {
            ConditionalEffect::Effect(_) => Err(()),
            ConditionalEffect::Forall(_) => Err(()),
            ConditionalEffect::When(x) => Ok(x),
        }
    }
}

impl TryInto<ForallConditionalEffect> for ConditionalEffect {
    type Error = ();

    fn try_into(self) -> Result<ForallConditionalEffect, Self::Error> {
        match self {
            ConditionalEffect::Effect(_) => Err(()),
            ConditionalEffect::Forall(x) => Ok(x),
            ConditionalEffect::When(_) => Err(()),
        }
    }
}

/// Alias for [`ConditionalEffect`]; matches BNF `<c-effect>`.
#[deprecated(since = "0.2.0", note = "Use `ConditionalEffect` instead")]
pub type CEffect = ConditionalEffect;

/// Alias for [`ForallConditionalEffect`]; matches BNF `<c-effect>` (forall variant).
#[deprecated(since = "0.2.0", note = "Use `ForallConditionalEffect` instead")]
pub type ForallCEffect = ForallConditionalEffect;

/// Alias for [`WhenConditionalEffect`]; matches BNF `<c-effect>` (when variant).
#[deprecated(since = "0.2.0", note = "Use `WhenConditionalEffect` instead")]
pub type WhenCEffect = WhenConditionalEffect;
