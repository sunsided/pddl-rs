//! Provides parsers for timed effects.

use crate::parsers::{parens, prefix_expr, ParseResult, Span};
use crate::parsers::{
    parse_assign_op_t, parse_effect_condition, parse_f_assign_da, parse_f_exp_t, parse_f_head,
    parse_time_specifier,
};
use crate::types::TimedEffect;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

/// Parses timed effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_timed_effect, preamble::*};
/// # use pddl::{AssignOp, TimedAssignOperator, AtomicFormula, EffectCondition, EqualityAtomicFormula, DurativeActionFunctionAssignment, DurativeActionFluentExpression, FunctionHead, PrimitiveEffect, Term, TimedEffect, TimedFluentExpression, TimeSpecifier};
/// # use pddl::DurativeActionFluentExpression::Duration;
/// assert!(parse_timed_effect("(at start (= x y))").is_value(
///     TimedEffect::new_conditional(
///         TimeSpecifier::Start,
///         EffectCondition::new(
///             PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Name("x".into()),
///                     Term::Name("y".into()))
///                 )
///             )
///         )
///     )
/// ));
///
/// assert!(parse_timed_effect("(at end (assign fun-sym ?duration))").is_value(
///     TimedEffect::new_fluent(
///         TimeSpecifier::End,
///         DurativeActionFunctionAssignment::new(
///             AssignOp::Assign,
///             FunctionHead::Simple("fun-sym".into()),
///             DurativeActionFluentExpression::Duration
///         )
///     )
/// ));
///
/// assert!(parse_timed_effect("(increase fun-sym #t)").is_value(
///     TimedEffect::new_continuous(
///         TimedAssignOperator::Increase,
///         FunctionHead::Simple("fun-sym".into()),
///         TimedFluentExpression::Now
///     )
/// ));
/// ```
pub fn parse_timed_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, TimedEffect> {
    let cond = map(
        prefix_expr(
            "at",
            (
                parse_time_specifier,
                preceded(multispace1, parse_effect_condition),
            ),
        ),
        TimedEffect::from,
    );

    // :numeric-fluents
    let fluent = map(
        prefix_expr(
            "at",
            (
                parse_time_specifier,
                preceded(multispace1, parse_f_assign_da),
            ),
        ),
        TimedEffect::from,
    );

    // :continuous-effects + :numeric-fluents
    let continuous = map(
        parens((
            parse_assign_op_t,
            preceded(multispace1, parse_f_head),
            preceded(multispace1, parse_f_exp_t),
        )),
        TimedEffect::from,
    );

    alt((fluent, cond, continuous)).parse(input.into())
}

/// Alias for [`parse_timed_effect`].
#[deprecated(since = "0.2.0", note = "Use `parse_timed_effect` instead")]
pub fn parse_timed_eff<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, TimedEffect> {
    parse_timed_effect(input)
}

impl crate::parsers::Parser for TimedEffect {
    type Item = TimedEffect;

    /// See [`parse_timed_effect`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_timed_effect(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::UnwrapValue;
    use crate::{
        AssignOp, TimedAssignOperator, AtomicFormula, EffectCondition, EqualityAtomicFormula, DurativeActionFunctionAssignment,
        DurativeActionFluentExpression, TimedFluentExpression, FunctionHead, PrimitiveEffect, Parser, Term, TimeSpecifier,
    };

    #[test]
    fn it_works() {
        let input = "(at end (increase (distance-travelled) 5))";
        let (_, _effect) = parse_timed_effect(Span::new(input)).unwrap();
    }

    #[test]
    fn test_parse() {
        assert!(
            TimedEffect::parse("(at start (= x y))").is_value(TimedEffect::new_conditional(
                TimeSpecifier::Start,
                EffectCondition::new(PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                )))
            ))
        );

        assert!(
            TimedEffect::parse("(at end (assign fun-sym ?duration))").is_value(
                TimedEffect::new_fluent(
                    TimeSpecifier::End,
                    DurativeActionFunctionAssignment::new(
                        AssignOp::Assign,
                        FunctionHead::Simple("fun-sym".into()),
                        DurativeActionFluentExpression::Duration
                    )
                )
            )
        );

        assert!(
            TimedEffect::parse("(increase fun-sym #t)").is_value(TimedEffect::new_continuous(
                TimedAssignOperator::Increase,
                FunctionHead::Simple("fun-sym".into()),
                TimedFluentExpression::Now
            ))
        );
    }
}
