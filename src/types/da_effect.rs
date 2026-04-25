//! Contains durative action effects via the [`DurativeActionEffect`] type.

use crate::types::TypedVariables;
use crate::types::{DurativeActionGoalDefinition, TimedEffect};

/// A durative action effect used in [`DurativeActionDefinition`](crate::types::DurativeActionDefinition).
///
/// ## Usage
/// Used by [`DurativeActionDefinition`](crate::DurativeActionDefinition).
#[derive(Debug, Clone, PartialEq)]
pub enum DurativeActionEffect {
    Timed(TimedEffect),
    /// Conjunction: All effects apply (i.e. a and b and c ..).
    All(Vec<DurativeActionEffect>),
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    Forall(TypedVariables, Box<DurativeActionEffect>),
    /// ## Requirements
    /// Requires [Conditional Effects](crate::Requirement::ConditionalEffects).
    When(DurativeActionGoalDefinition, TimedEffect),
}

impl DurativeActionEffect {
    #[doc(alias = "new_timed")]
    pub const fn timed(effect: TimedEffect) -> Self {
        Self::Timed(effect)
    }

    pub fn new_timed(effect: TimedEffect) -> Self {
        Self::timed(effect)
    }

    #[doc(alias = "new_and")]
    pub fn and<E: IntoIterator<Item = DurativeActionEffect>>(effect: E) -> Self {
        Self::All(effect.into_iter().collect())
    }

    pub fn new_and<E: IntoIterator<Item = DurativeActionEffect>>(effect: E) -> Self {
        Self::and(effect)
    }

    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, effect: DurativeActionEffect) -> Self {
        Self::Forall(variables, Box::new(effect))
    }

    pub fn new_forall(variables: TypedVariables, effect: DurativeActionEffect) -> Self {
        Self::r#forall(variables, effect)
    }

    #[doc(alias = "new_when")]
    pub const fn when(gd: DurativeActionGoalDefinition, effect: TimedEffect) -> Self {
        Self::When(gd, effect)
    }

    pub fn new_when(gd: DurativeActionGoalDefinition, effect: TimedEffect) -> Self {
        Self::when(gd, effect)
    }
}

impl From<TimedEffect> for DurativeActionEffect {
    fn from(value: TimedEffect) -> Self {
        Self::timed(value)
    }
}

impl FromIterator<DurativeActionEffect> for DurativeActionEffect {
    fn from_iter<T: IntoIterator<Item = DurativeActionEffect>>(iter: T) -> Self {
        Self::and(iter)
    }
}

impl From<(TypedVariables, DurativeActionEffect)> for DurativeActionEffect {
    fn from(value: (TypedVariables, DurativeActionEffect)) -> Self {
        Self::r#forall(value.0, value.1)
    }
}

impl From<(DurativeActionGoalDefinition, TimedEffect)> for DurativeActionEffect {
    fn from(value: (DurativeActionGoalDefinition, TimedEffect)) -> Self {
        Self::when(value.0, value.1)
    }
}
