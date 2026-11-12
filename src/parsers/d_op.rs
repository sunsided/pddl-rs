//! Provides parsers for durative operations.

use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::value;
use nom::Parser;

use crate::parsers::{ParseResult, Span};
use crate::types::{d_op::names, DurationOperator};

/// Parses a durative operation, i.e. `<= | >= | =`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_d_op, preamble::*};
/// # use pddl::{DurationOperator};
/// assert!(parse_d_op("<=").is_value(DurationOperator::LessThanOrEqual));
/// assert!(parse_d_op(">=").is_value(DurationOperator::GreaterOrEqual));
/// assert!(parse_d_op("=").is_value(DurationOperator::Equal));
///```
pub fn parse_d_op<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, DurationOperator> {
    // :duration-inequalities
    alt((
        value(
            DurationOperator::LessThanOrEqual,
            tag_no_case(names::LESS_THAN_OR_EQUAL),
        ),
        value(
            DurationOperator::GreaterOrEqual,
            tag_no_case(names::GREATER_OR_EQUAL),
        ),
        value(DurationOperator::Equal, tag_no_case(names::EQUAL)),
    ))
    .parse(input.into())
}

impl crate::parsers::Parser for DurationOperator {
    type Item = DurationOperator;

    /// Parses a durative operation.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{DurationOperator, Parser};
    /// let (_, value) = DurationOperator::parse("<=").unwrap();
    /// assert_eq!(value, DurationOperator::LessThanOrEqual);
    ///```
    ///
    /// ## See also
    /// See [`parse_d_op`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_d_op(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{DurationOperator, Parser};

    #[test]
    fn test_parse() {
        assert!(DurationOperator::parse("<=").is_value(DurationOperator::LessThanOrEqual));
        assert!(DurationOperator::parse(">=").is_value(DurationOperator::GreaterOrEqual));
        assert!(DurationOperator::parse("=").is_value(DurationOperator::Equal));
    }
}
