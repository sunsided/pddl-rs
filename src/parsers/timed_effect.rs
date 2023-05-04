//! Provides parsers for timed effects.

use crate::parsers::{parens, prefix_expr, ParseResult, Span};
use crate::parsers::{
    parse_assign_op_t, parse_cond_effect, parse_f_assign_da, parse_f_exp_t, parse_f_head,
    parse_time_specifier,
};
use crate::types::TimedEffect;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parser that parses timed effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_timed_effect, preamble::*};
/// # use pddl::{AssignOp, AssignOpT, AtomicFormula, CEffect, ConditionalEffect, EqualityAtomicFormula, FAssignDa, FExpDa, FExpT, FHead, PEffect, Term, TimedEffect, TimeSpecifier};
/// # use pddl::FExpDa::FExp;
/// assert!(parse_timed_effect("(at start (= x y))".into()).is_value(
///     TimedEffect::new_conditional(
///         TimeSpecifier::Start,
///         ConditionalEffect::new(
///             PEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Name("x".into()),
///                     Term::Name("y".into()))
///                 )
///             )
///         )
///     )
/// ));
///
/// assert!(parse_timed_effect("(at end (assign fun-sym ?duration))".into()).is_value(
///     TimedEffect::new_fluent(
///         TimeSpecifier::End,
///         FAssignDa::new(
///             AssignOp::Assign,
///             FHead::Simple("fun-sym".into()),
///             FExpDa::Duration
///         )
///     )
/// ));
///
/// assert!(parse_timed_effect("(increase fun-sym #t)".into()).is_value(
///     TimedEffect::new_continuous(
///         AssignOpT::Increase,
///         FHead::Simple("fun-sym".into()),
///         FExpT::Now
///     )
/// ));
/// ```
pub fn parse_timed_effect(input: Span) -> ParseResult<TimedEffect> {
    let cond = map(
        prefix_expr(
            "at",
            tuple((
                parse_time_specifier,
                preceded(multispace1, parse_cond_effect),
            )),
        ),
        TimedEffect::from,
    );

    // :numeric-fluents
    let fluent = map(
        prefix_expr(
            "at",
            tuple((
                parse_time_specifier,
                preceded(multispace1, parse_f_assign_da),
            )),
        ),
        TimedEffect::from,
    );

    // :continuous-effects + :numeric-fluents
    let continuous = map(
        parens(tuple((
            parse_assign_op_t,
            preceded(multispace1, parse_f_head),
            preceded(multispace1, parse_f_exp_t),
        ))),
        TimedEffect::from,
    );

    alt((fluent, cond, continuous))(input)
}

impl<'a> crate::parsers::Parser<'a> for TimedEffect<'a> {
    type Item = TimedEffect<'a>;

    /// See [`parse_timed_effect`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_timed_effect(input.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "(at end (increase (distance-travelled) 5))";
        let (_, _effect) = parse_timed_effect(Span::new(input)).unwrap();
    }
}
