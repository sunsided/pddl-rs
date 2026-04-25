//! Contains the [`TimedEffect`] type.

use crate::types::{EffectCondition, TimedAssignOperator, DurativeActionFunctionAssignment, TimedFluentExpression, FunctionHead, TimeSpecifier};

/// A timed effect, either conditional, continuous or derived from a fluent, e.g. [`DurativeActionEffect`](crate::types::DurativeActionEffect).
///
/// An effect is a condition which is made true when an action is applied.
/// Note that the effect is always more restrictive than an action and typically only
/// allows `and` and `not` as logical expressions.
///
/// ## Notes
///
/// Temporal expressions, such as `at start` and `at end` are available, however, `over all`
/// is typically not used because it's not common to express a boolean effect which is true
/// over the duration of the action.
///
/// Instead you would set it to true at the start, using an `at start` and set it to false at
/// the end using `at end`.
///
/// ## Usage
/// Used by [`DurativeActionEffect`](crate::DurativeActionEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum TimedEffect {
    Conditional(TimeSpecifier, EffectCondition),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    NumericFluent(TimeSpecifier, DurativeActionFunctionAssignment),
    /// ## Requirements
    /// Requires [Continuous Effects](crate::Requirement::ContinuousEffects) and
    /// [Numeric Fluents](crate::Requirement::NumericFluents).
    ContinuousEffect(TimedAssignOperator, FunctionHead, TimedFluentExpression),
}

impl TimedEffect {
    pub const fn new_conditional(at: TimeSpecifier, effect: EffectCondition) -> Self {
        Self::Conditional(at, effect)
    }

    pub const fn new_fluent(at: TimeSpecifier, action: DurativeActionFunctionAssignment) -> Self {
        Self::NumericFluent(at, action)
    }

    pub const fn new_continuous(operation: TimedAssignOperator, f_head: FunctionHead, f_exp_t: TimedFluentExpression) -> Self {
        Self::ContinuousEffect(operation, f_head, f_exp_t)
    }
}

impl From<(TimeSpecifier, EffectCondition)> for TimedEffect {
    fn from(value: (TimeSpecifier, EffectCondition)) -> Self {
        TimedEffect::Conditional(value.0, value.1)
    }
}

impl From<(TimeSpecifier, DurativeActionFunctionAssignment)> for TimedEffect {
    fn from(value: (TimeSpecifier, DurativeActionFunctionAssignment)) -> Self {
        TimedEffect::NumericFluent(value.0, value.1)
    }
}

impl From<(TimedAssignOperator, FunctionHead, TimedFluentExpression)> for TimedEffect {
    fn from(value: (TimedAssignOperator, FunctionHead, TimedFluentExpression)) -> Self {
        TimedEffect::ContinuousEffect(value.0, value.1, value.2)
    }
}
