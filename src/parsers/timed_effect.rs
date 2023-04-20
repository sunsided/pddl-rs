//! Provides parsers for timed effects.

use crate::parsers::{
    parens, parse_assign_op_t, parse_cond_effect, parse_f_assign_da, parse_f_exp_t, parse_f_head,
    parse_time_specifier, prefix_expr,
};
use crate::types::TimedEffect;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser that parses timed effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_timed_effect;
/// # use pddl::types::{AssignOp, AssignOpT, AtomicFormula, CEffect, ConditionalEffect, EqualityAtomicFormula, FAssignDa, FExpDa, FExpT, FHead, PEffect, Term, TimedEffect, TimeSpecifier};
/// # use pddl::types::FExpDa::FExp;
/// assert_eq!(parse_timed_effect("(at start (= x y))"), Ok(("",
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
/// )));
///
/// assert_eq!(parse_timed_effect("(at end (assign fun-sym ?duration))"), Ok(("",
///     TimedEffect::new_fluent(
///         TimeSpecifier::End,
///         FAssignDa::new(
///             AssignOp::Assign,
///             FHead::Simple("fun-sym".into()),
///             FExpDa::Duration
///         )
///     )
/// )));
///
/// assert_eq!(parse_timed_effect("(increase fun-sym #t)"), Ok(("",
///     TimedEffect::new_continuous(
///         AssignOpT::Increase,
///         FHead::Simple("fun-sym".into()),
///         FExpT::Now
///     )
/// )));
/// ```
pub fn parse_timed_effect(input: &str) -> IResult<&str, TimedEffect> {
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
