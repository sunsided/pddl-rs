//! Provides parsers for duration constraints.

use crate::parsers::parse_simple_duration_constraint;
use crate::parsers::{prefix_expr, space_separated_list1};
use crate::types::DurationConstraint;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

/// Parses a duration constraint.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_duration_constraint;
/// # use pddl::types::{DOp, DurationConstraint, DValue, FunctionType, SimpleDurationConstraint, TimeSpecifier};
///
/// let input = "()";
/// assert_eq!(parse_duration_constraint(input), Ok(("", None)));
///
/// let input = "(= ?duration 5)";
/// assert_eq!(parse_duration_constraint(input), Ok(("",
///     Some(DurationConstraint::new_simple(
///         SimpleDurationConstraint::Op(
///             DOp::Equal,
///             DValue::Number(5.into())
///         )
///     ))
/// )));
///
/// let input = "(at end (<= ?duration 1.23))";
/// assert_eq!(parse_duration_constraint(input), Ok(("",
///     Some(DurationConstraint::new_simple(
///         SimpleDurationConstraint::new_at(
///             TimeSpecifier::End,
///             SimpleDurationConstraint::Op(
///                 DOp::LessThanOrEqual,
///                 DValue::Number(1.23.into())
///             )
///         )
///     ))
/// )));
///
/// let input = "(and (at end (<= ?duration 1.23)) (>= ?duration 1.0))";
/// assert_eq!(parse_duration_constraint(input), Ok(("",
///     Some(DurationConstraint::new_all([
///         SimpleDurationConstraint::new_at(
///             TimeSpecifier::End,
///             SimpleDurationConstraint::Op(
///                 DOp::LessThanOrEqual,
///                 DValue::Number(1.23.into())
///             )
///         ),
///         SimpleDurationConstraint::new_op(
///             DOp::GreaterOrEqual,
///             DValue::Number(1.0.into())
///         )
///     ]))
/// )));
///```
pub fn parse_duration_constraint(input: &str) -> IResult<&str, Option<DurationConstraint>> {
    let none = map(tag("()"), |_| None);
    let simple = map(parse_simple_duration_constraint, |c| {
        Some(DurationConstraint::from(c))
    });

    // : duration-inequalities
    let and = map(
        prefix_expr(
            "and",
            space_separated_list1(parse_simple_duration_constraint),
        ),
        |cs| Some(DurationConstraint::from_iter(cs)),
    );

    alt((none, simple, and))(input)
}
