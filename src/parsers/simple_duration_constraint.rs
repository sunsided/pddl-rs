//! Provides parsers for simple duration constraints.

use crate::parsers::{parens, prefix_expr};
use crate::parsers::{parse_d_op, parse_d_value, parse_time_specifier};
use crate::types::SimpleDurationConstraint;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a simple duration constraint.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_simple_duration_constraint;
/// # use pddl::types::{DOp, DurationValue, FunctionType, SimpleDurationConstraint, TimeSpecifier};
///
/// let input = "(>= ?duration 1.23)";
/// assert_eq!(parse_simple_duration_constraint(input), Ok(("",
///     SimpleDurationConstraint::new_op(
///         DOp::GreaterOrEqual,
///         DurationValue::new_number(1.23)
///     )
/// )));
///
/// let input = "(at end (<= ?duration 1.23))";
/// assert_eq!(parse_simple_duration_constraint(input), Ok(("",
///     SimpleDurationConstraint::new_at(
///         TimeSpecifier::End,
///         SimpleDurationConstraint::Op(
///             DOp::LessThanOrEqual,
///             DurationValue::new_number(1.23)
///         )
///     )
/// )));
///```
pub fn parse_simple_duration_constraint(input: &str) -> IResult<&str, SimpleDurationConstraint> {
    let op = map(
        parens(tuple((
            parse_d_op,
            preceded(
                tuple((multispace1, tag("?duration"), multispace1)),
                parse_d_value,
            ),
        ))),
        SimpleDurationConstraint::from,
    );

    let at = map(
        prefix_expr(
            "at",
            tuple((
                parse_time_specifier,
                preceded(multispace1, parse_simple_duration_constraint),
            )),
        ),
        SimpleDurationConstraint::from,
    );

    alt((op, at))(input)
}

impl<'a> crate::parsers::Parser<'a> for SimpleDurationConstraint<'a> {
    type Item = SimpleDurationConstraint<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_simple_duration_constraint(input)
    }
}
