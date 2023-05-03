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
pub enum TimedEffect<'a> {
    Conditional(TimeSpecifier, ConditionalEffect<'a>),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    NumericFluent(TimeSpecifier, FAssignDa<'a>),
    /// ## Requirements
    /// Requires [Continuous Effects](crate::Requirement::ContinuousEffects) and
    /// [Numeric Fluents](crate::Requirement::NumericFluents).
    ContinuousEffect(AssignOpT, FHead<'a>, FExpT<'a>),
}

impl<'a> TimedEffect<'a> {
    pub const fn new_conditional(at: TimeSpecifier, effect: ConditionalEffect<'a>) -> Self {
        Self::Conditional(at, effect)
    }

    pub const fn new_fluent(at: TimeSpecifier, action: FAssignDa<'a>) -> Self {
        Self::NumericFluent(at, action)
    }

    pub const fn new_continuous(
        operation: AssignOpT,
        f_head: FHead<'a>,
        f_exp_t: FExpT<'a>,
    ) -> Self {
        Self::ContinuousEffect(operation, f_head, f_exp_t)
    }
}

impl<'a> From<(TimeSpecifier, ConditionalEffect<'a>)> for TimedEffect<'a> {
    fn from(value: (TimeSpecifier, ConditionalEffect<'a>)) -> Self {
        TimedEffect::Conditional(value.0, value.1)
    }
}

impl<'a> From<(TimeSpecifier, FAssignDa<'a>)> for TimedEffect<'a> {
    fn from(value: (TimeSpecifier, FAssignDa<'a>)) -> Self {
        TimedEffect::NumericFluent(value.0, value.1)
    }
}

impl<'a> From<(AssignOpT, FHead<'a>, FExpT<'a>)> for TimedEffect<'a> {
    fn from(value: (AssignOpT, FHead<'a>, FExpT<'a>)) -> Self {
        TimedEffect::ContinuousEffect(value.0, value.1, value.2)
    }
}
