//! Provides parsers for binary comparison operations.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::Parser;

use crate::parsers::{ParseResult, Span};
use crate::types::{binary_comp::names, BinaryComparison};

/// Parses a binary comparison operation.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_binary_comp, Span, UnwrapValue};
/// # use pddl::{AssignOp, BinaryComparison};
/// assert!(parse_binary_comp(Span::new(">")).is_value(BinaryComparison::GreaterThan));
/// assert!(parse_binary_comp(Span::new("<")).is_value(BinaryComparison::LessThan));
/// assert!(parse_binary_comp(Span::new("=")).is_value(BinaryComparison::Equal));
/// assert!(parse_binary_comp(Span::new(">=")).is_value(BinaryComparison::GreaterOrEqual));
/// assert!(parse_binary_comp(Span::new("<=")).is_value(BinaryComparison::LessThanOrEqual));
///```
pub fn parse_binary_comp<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, BinaryComparison> {
    map(
        alt((
            tag(names::GREATER_THAN_OR_EQUAL),
            tag(names::LESS_THAN_OR_EQUAL),
            tag(names::EQUAL),
            tag(names::GREATER_THAN),
            tag(names::LESS_THAN),
        )),
        |x: Span| BinaryComparison::try_from(*x.fragment()).expect("unhandled variant"),
    )
    .parse(input.into())
}

impl crate::parsers::Parser for BinaryComparison {
    type Item = BinaryComparison;

    /// Parses a binary comparison operation.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{BinaryComparison, Parser};
    /// let (_, value) = BinaryComparison::parse(">=").unwrap();
    /// assert_eq!(value, BinaryComparison::GreaterOrEqual);
    ///```
    ///
    /// ## See also
    /// See [`parse_binary_comp`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_binary_comp(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BinaryComparison, Parser};

    #[test]
    fn test_parse() {
        let (_, value) = BinaryComparison::parse(">=").unwrap();
        assert_eq!(value, BinaryComparison::GreaterOrEqual);

        let (_, value) = BinaryComparison::parse(">").unwrap();
        assert_eq!(value, BinaryComparison::GreaterThan);

        let (_, value) = BinaryComparison::parse("<=").unwrap();
        assert_eq!(value, BinaryComparison::LessThanOrEqual);

        let (_, value) = BinaryComparison::parse("<").unwrap();
        assert_eq!(value, BinaryComparison::LessThan);

        let (_, value) = BinaryComparison::parse("=").unwrap();
        assert_eq!(value, BinaryComparison::Equal);
    }
}
