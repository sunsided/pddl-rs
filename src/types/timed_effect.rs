//! Contains the [`TimedEffect`] type.

use crate::types::{
    DurativeActionFunctionAssignment, EffectCondition, FunctionHead, TimeSpecifier,
    TimedAssignOperator, TimedFluentExpression,
};

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
    #[doc(alias = "new_conditional")]
    pub const fn conditional(at: TimeSpecifier, effect: EffectCondition) -> Self {
        Self::Conditional(at, effect)
    }

    #[doc(alias = "new_fluent")]
    pub const fn fluent(at: TimeSpecifier, action: DurativeActionFunctionAssignment) -> Self {
        Self::NumericFluent(at, action)
    }

    #[doc(alias = "new_continuous")]
    pub const fn continuous(
        operation: TimedAssignOperator,
        f_head: FunctionHead,
        f_exp_t: TimedFluentExpression,
    ) -> Self {
        Self::ContinuousEffect(operation, f_head, f_exp_t)
    }

    #[deprecated(since = "0.2.0", note = "Use `conditional` instead")]
    pub const fn new_conditional(at: TimeSpecifier, effect: EffectCondition) -> Self {
        Self::conditional(at, effect)
    }

    #[deprecated(since = "0.2.0", note = "Use `fluent` instead")]
    pub const fn new_fluent(at: TimeSpecifier, action: DurativeActionFunctionAssignment) -> Self {
        Self::fluent(at, action)
    }

    #[deprecated(since = "0.2.0", note = "Use `continuous` instead")]
    pub const fn new_continuous(
        operation: TimedAssignOperator,
        f_head: FunctionHead,
        f_exp_t: TimedFluentExpression,
    ) -> Self {
        Self::continuous(operation, f_head, f_exp_t)
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AssignOp, AtomicFormula, DurativeActionFluentExpression, DurativeActionFunctionAssignment,
        EffectCondition, FluentExpression, FunctionSymbol, Name, Predicate, PrimitiveEffect, Term,
    };

    fn make_effect_condition() -> EffectCondition {
        let af = AtomicFormula::<Term>::predicate(
            Predicate::string("on"),
            vec![Term::new_name(Name::new("a"))],
        );
        EffectCondition::new(PrimitiveEffect::atomic_formula(af))
    }

    #[test]
    fn conditional_at_start() {
        let ec = make_effect_condition();
        let te = TimedEffect::conditional(TimeSpecifier::Start, ec);
        assert!(matches!(
            te,
            TimedEffect::Conditional(TimeSpecifier::Start, _)
        ));
    }

    #[test]
    fn conditional_at_end() {
        let ec = make_effect_condition();
        let te = TimedEffect::conditional(TimeSpecifier::End, ec);
        assert!(matches!(
            te,
            TimedEffect::Conditional(TimeSpecifier::End, _)
        ));
    }

    #[test]
    fn fluent_assignment() {
        let fa = DurativeActionFunctionAssignment::new(
            AssignOp::Assign,
            FunctionHead::Simple(FunctionSymbol::string("cost")),
            DurativeActionFluentExpression::FluentExpression(FluentExpression::number(10)),
        );
        let te = TimedEffect::fluent(TimeSpecifier::End, fa);
        assert!(matches!(
            te,
            TimedEffect::NumericFluent(TimeSpecifier::End, _)
        ));
    }

    #[test]
    fn continuous_effect() {
        let op = TimedAssignOperator::Increase;
        let head = FunctionHead::Simple(FunctionSymbol::string("battery"));
        let exp = TimedFluentExpression::scaled(FluentExpression::number(1));
        let te = TimedEffect::continuous(op, head, exp);
        assert!(matches!(te, TimedEffect::ContinuousEffect(_, _, _)));
    }

    #[test]
    fn from_conditional_tuple() {
        let ec = make_effect_condition();
        let te = TimedEffect::from((TimeSpecifier::Start, ec));
        assert!(matches!(
            te,
            TimedEffect::Conditional(TimeSpecifier::Start, _)
        ));
    }

    #[test]
    fn from_fluent_tuple() {
        let fa = DurativeActionFunctionAssignment::new(
            AssignOp::Assign,
            FunctionHead::Simple(FunctionSymbol::string("cost")),
            DurativeActionFluentExpression::FluentExpression(FluentExpression::number(10)),
        );
        let te = TimedEffect::from((TimeSpecifier::End, fa));
        assert!(matches!(
            te,
            TimedEffect::NumericFluent(TimeSpecifier::End, _)
        ));
    }

    #[test]
    fn from_continuous_tuple() {
        let op = TimedAssignOperator::Increase;
        let head = FunctionHead::Simple(FunctionSymbol::string("battery"));
        let exp = TimedFluentExpression::scaled(FluentExpression::number(1));
        let te = TimedEffect::from((op, head, exp));
        assert!(matches!(te, TimedEffect::ContinuousEffect(_, _, _)));
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_conditional() {
        let ec = make_effect_condition();
        let te = TimedEffect::new_conditional(TimeSpecifier::Start, ec);
        assert!(matches!(
            te,
            TimedEffect::Conditional(TimeSpecifier::Start, _)
        ));
    }

    #[test]
    fn clone_works() {
        let ec = make_effect_condition();
        let te = TimedEffect::conditional(TimeSpecifier::Start, ec);
        let clone = te.clone();
        assert_eq!(te, clone);
    }

    #[test]
    fn debug_impl() {
        let ec = make_effect_condition();
        let te = TimedEffect::conditional(TimeSpecifier::Start, ec);
        let dbg = format!("{te:?}");
        assert!(dbg.contains("Conditional"));
    }
}
