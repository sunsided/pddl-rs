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
    pub const fn new_timed(effect: TimedEffect) -> Self {
        Self::Timed(effect)
    }
    pub fn new_and<E: IntoIterator<Item = DurativeActionEffect>>(effect: E) -> Self {
        Self::All(effect.into_iter().collect())
    }
    pub fn new_forall(variables: TypedVariables, effect: DurativeActionEffect) -> Self {
        Self::Forall(variables, Box::new(effect))
    }
    pub const fn new_when(gd: DurativeActionGoalDefinition, effect: TimedEffect) -> Self {
        Self::When(gd, effect)
    }
}

impl From<TimedEffect> for DurativeActionEffect {
    fn from(value: TimedEffect) -> Self {
        Self::new_timed(value)
    }
}

impl FromIterator<DurativeActionEffect> for DurativeActionEffect {
    fn from_iter<T: IntoIterator<Item = DurativeActionEffect>>(iter: T) -> Self {
        Self::new_and(iter)
    }
}

impl From<(TypedVariables, DurativeActionEffect)> for DurativeActionEffect {
    fn from(value: (TypedVariables, DurativeActionEffect)) -> Self {
        Self::new_forall(value.0, value.1)
    }
}

impl From<(DurativeActionGoalDefinition, TimedEffect)> for DurativeActionEffect {
    fn from(value: (DurativeActionGoalDefinition, TimedEffect)) -> Self {
        Self::new_when(value.0, value.1)
    }
}
