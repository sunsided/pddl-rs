//! Provides parsers for simple duration constraints.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

use crate::parsers::{parens, prefix_expr, ParseResult, Span};
use crate::parsers::{parse_d_op, parse_d_value, parse_time_specifier};
use crate::types::SimpleDurationConstraint;

/// Parses a simple duration constraint.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_simple_duration_constraint, preamble::*};
/// # use pddl::{DOp, DurationValue, FunctionType, SimpleDurationConstraint, TimeSpecifier};
/// let input = "(>= ?duration 1.23)";
/// assert!(parse_simple_duration_constraint(input).is_value(
///     SimpleDurationConstraint::new_op(
///         DOp::GreaterOrEqual,
///         DurationValue::new_number(1.23)
///     )
/// ));
///
/// let input = "(at end (<= ?duration 1.23))";
/// assert!(parse_simple_duration_constraint(input).is_value(
///     SimpleDurationConstraint::new_at(
///         TimeSpecifier::End,
///         SimpleDurationConstraint::Op(
///             DOp::LessThanOrEqual,
///             DurationValue::new_number(1.23)
///         )
///     )
/// ));
///```
pub fn parse_simple_duration_constraint<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, SimpleDurationConstraint> {
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

    alt((op, at))(input.into())
}

impl crate::parsers::Parser for SimpleDurationConstraint {
    type Item = SimpleDurationConstraint;

    /// See [`parse_simple_duration_constraint`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_simple_duration_constraint(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{DOp, DurationValue, SimpleDurationConstraint, TimeSpecifier};

    #[test]
    fn test_parse() {
        let input = "(>= ?duration 1.23)";
        assert!(
            SimpleDurationConstraint::parse(input).is_value(SimpleDurationConstraint::new_op(
                DOp::GreaterOrEqual,
                DurationValue::new_number(1.23)
            ))
        );

        let input = "(at end (<= ?duration 1.23))";
        assert!(
            SimpleDurationConstraint::parse(input).is_value(SimpleDurationConstraint::new_at(
                TimeSpecifier::End,
                SimpleDurationConstraint::Op(DOp::LessThanOrEqual, DurationValue::new_number(1.23))
            ))
        );
    }
}
