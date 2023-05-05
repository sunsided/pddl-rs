//! Contains the [`TimedEffect`] type.

use crate::types::{AssignOpT, ConditionalEffect, FAssignDa, FExpT, FHead, TimeSpecifier};

/// A timed effect, either conditional, continuous or derived from a fluent, e.g. [`DurativeActionEffect`](crate::types::DurativeActionEffect).
///
/// An effect is a condition which is made true when an action is applied.
/// Note that the effect is always more restrictive than an action and typically only
/// allows `and` and `not` as logical expressions.
///
/// ## Notes
///
/// Temporal expressions, such as `at start` and `at end` are available, however, `over all`
/// is typically not used because it’s not common to express a boolean effect which is true
/// over the duration of the action.
///
/// Instead you would set it to true at the start, using an `at start` and set it to false at
/// the end using `at end`.
///
/// ## Usage
/// Used by [`DurativeActionEffect`](crate::DurativeActionEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum TimedEffect {
    Conditional(TimeSpecifier, ConditionalEffect),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    NumericFluent(TimeSpecifier, FAssignDa),
    /// ## Requirements
    /// Requires [Continuous Effects](crate::Requirement::ContinuousEffects) and
    /// [Numeric Fluents](crate::Requirement::NumericFluents).
    ContinuousEffect(AssignOpT, FHead, FExpT),
}

impl TimedEffect {
    pub const fn new_conditional(at: TimeSpecifier, effect: ConditionalEffect) -> Self {
        Self::Conditional(at, effect)
    }

    pub const fn new_fluent(at: TimeSpecifier, action: FAssignDa) -> Self {
        Self::NumericFluent(at, action)
    }

    pub const fn new_continuous(operation: AssignOpT, f_head: FHead, f_exp_t: FExpT) -> Self {
        Self::ContinuousEffect(operation, f_head, f_exp_t)
    }
}

impl From<(TimeSpecifier, ConditionalEffect)> for TimedEffect {
    fn from(value: (TimeSpecifier, ConditionalEffect)) -> Self {
        TimedEffect::Conditional(value.0, value.1)
    }
}

impl From<(TimeSpecifier, FAssignDa)> for TimedEffect {
    fn from(value: (TimeSpecifier, FAssignDa)) -> Self {
        TimedEffect::NumericFluent(value.0, value.1)
    }
}

impl From<(AssignOpT, FHead, FExpT)> for TimedEffect {
    fn from(value: (AssignOpT, FHead, FExpT)) -> Self {
        TimedEffect::ContinuousEffect(value.0, value.1, value.2)
    }
}
