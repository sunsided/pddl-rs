//! Provides parsers for binary comparison operations.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

use crate::parsers::{ParseResult, Span};
use crate::types::{binary_comp::names, BinaryComp};

/// Parses a binary comparison operation.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_binary_comp, Span, UnwrapValue};
/// # use pddl::{AssignOp, BinaryComp};
/// assert!(parse_binary_comp(Span::new(">")).is_value(BinaryComp::GreaterThan));
/// assert!(parse_binary_comp(Span::new("<")).is_value(BinaryComp::LessThan));
/// assert!(parse_binary_comp(Span::new("=")).is_value(BinaryComp::Equal));
/// assert!(parse_binary_comp(Span::new(">=")).is_value(BinaryComp::GreaterOrEqual));
/// assert!(parse_binary_comp(Span::new("<=")).is_value(BinaryComp::LessThanOrEqual));
///```
pub fn parse_binary_comp<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, BinaryComp> {
    map(
        alt((
            tag(names::GREATER_THAN_OR_EQUAL),
            tag(names::LESS_THAN_OR_EQUAL),
            tag(names::EQUAL),
            tag(names::GREATER_THAN),
            tag(names::LESS_THAN),
        )),
        |x: Span| BinaryComp::try_from(*x.fragment()).expect("unhandled variant"),
    )(input.into())
}

impl crate::parsers::Parser for BinaryComp {
    type Item = BinaryComp;

    /// Parses a binary comparison operation.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{BinaryComp, Parser};
    /// let (_, value) = BinaryComp::parse(">=").unwrap();
    /// assert_eq!(value, BinaryComp::GreaterOrEqual);
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
    use crate::{BinaryComp, Parser};

    #[test]
    fn test_parse() {
        let (_, value) = BinaryComp::parse(">=").unwrap();
        assert_eq!(value, BinaryComp::GreaterOrEqual);

        let (_, value) = BinaryComp::parse(">").unwrap();
        assert_eq!(value, BinaryComp::GreaterThan);

        let (_, value) = BinaryComp::parse("<=").unwrap();
        assert_eq!(value, BinaryComp::LessThanOrEqual);

        let (_, value) = BinaryComp::parse("<").unwrap();
        assert_eq!(value, BinaryComp::LessThan);

        let (_, value) = BinaryComp::parse("=").unwrap();
        assert_eq!(value, BinaryComp::Equal);
    }
}
