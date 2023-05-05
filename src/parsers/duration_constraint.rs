//! Provides parsers for duration constraints.

use crate::parsers::{parse_simple_duration_constraint, ParseResult, Span};
use crate::parsers::{prefix_expr, space_separated_list1};
use crate::types::DurationConstraint;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses a duration constraint.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_duration_constraint, preamble::*};
/// # use pddl::{DOp, DurationConstraint, DurationValue, FunctionType, SimpleDurationConstraint, TimeSpecifier};
/// let input = "()";
/// assert!(parse_duration_constraint(input).is_value(None));
///
/// let input = "(= ?duration 5)";
/// assert!(parse_duration_constraint(input).is_value(
///     Some(DurationConstraint::new(
///         SimpleDurationConstraint::Op(
///             DOp::Equal,
///             DurationValue::Number(5.into())
///         )
///     ))
/// ));
///
/// let input = "(at end (<= ?duration 1.23))";
/// assert!(parse_duration_constraint(input).is_value(
///     Some(DurationConstraint::new(
///         SimpleDurationConstraint::new_at(
///             TimeSpecifier::End,
///             SimpleDurationConstraint::Op(
///                 DOp::LessThanOrEqual,
///                 DurationValue::Number(1.23.into())
///             )
///         )
///     ))
/// ));
///
/// let input = "(and (at end (<= ?duration 1.23)) (>= ?duration 1.0))";
/// assert!(parse_duration_constraint(input).is_value(
///     Some(DurationConstraint::new_all([
///         SimpleDurationConstraint::new_at(
///             TimeSpecifier::End,
///             SimpleDurationConstraint::Op(
///                 DOp::LessThanOrEqual,
///                 DurationValue::Number(1.23.into())
///             )
///         ),
///         SimpleDurationConstraint::new_op(
///             DOp::GreaterOrEqual,
///             DurationValue::Number(1.0.into())
///         )
///     ]))
/// ));
///```
pub fn parse_duration_constraint<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, Option<DurationConstraint>> {
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

    alt((none, simple, and))(input.into())
}

impl crate::parsers::Parser for DurationConstraint {
    type Item = Option<DurationConstraint>;

    /// See [`parse_duration_constraint`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_duration_constraint(input)
    }
}
