//! Contains the [`TimedEffect`] type.

use crate::types::{AssignOpT, ConditionalEffect, FAssignDa, FExpT, FHead, TimeSpecifier};

#[derive(Debug, Clone, PartialEq)]
pub enum TimedEffect<'a> {
    ConditionalEffect(TimeSpecifier, ConditionalEffect<'a>),
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    NumericFluent(TimeSpecifier, FAssignDa<'a>),
    /// Requires [ContinuousEffects](crate::types::Requirement::ContinuousEffects) and
    /// [NumericFluents](crate::types::Requirement::NumericFluents).
    ContinuousEffect(AssignOpT, FHead<'a>, FExpT<'a>),
}

impl<'a> TimedEffect<'a> {
    pub const fn new_conditional(at: TimeSpecifier, effect: ConditionalEffect<'a>) -> Self {
        Self::ConditionalEffect(at, effect)
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
        TimedEffect::ConditionalEffect(value.0, value.1)
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
