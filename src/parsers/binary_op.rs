//! Provides parsers for binary-operand operations.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

use crate::parsers::{ParseResult, Span};
use crate::types::{binary_op::names, BinaryOp};

/// Parses a two-operand operation, i.e. `* | + | - | /`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_binary_op, Span, UnwrapValue};
/// # use pddl::{BinaryOp};
/// assert!(parse_binary_op(Span::new("*")).is_value(BinaryOp::Multiplication));
/// assert!(parse_binary_op(Span::new("+")).is_value(BinaryOp::Addition));
/// assert!(parse_binary_op(Span::new("-")).is_value(BinaryOp::Subtraction));
/// assert!(parse_binary_op(Span::new("/")).is_value(BinaryOp::Division));
///```
pub fn parse_binary_op<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, BinaryOp> {
    map(
        alt((
            tag(names::MULTIPLICATION),
            tag(names::ADDITION),
            tag(names::SUBTRACTION),
            tag(names::DIVISION),
        )),
        |x: Span| BinaryOp::try_from(*x.fragment()).expect("unhandled variant"),
    )(input.into())
}

impl crate::parsers::Parser for BinaryOp {
    type Item = BinaryOp;

    /// Parses a two-operand operation.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{BinaryOp, Parser};
    /// let (_, value) = BinaryOp::parse("*").unwrap();
    /// assert_eq!(value, BinaryOp::Multiplication);
    ///```
    ///
    /// ## See also.
    /// See [`parse_binary_op`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_binary_op(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BinaryOp, Parser};

    #[test]
    fn test_parse() {
        let (_, value) = BinaryOp::parse("*").unwrap();
        assert_eq!(value, BinaryOp::Multiplication);

        let (_, value) = BinaryOp::parse("/").unwrap();
        assert_eq!(value, BinaryOp::Division);

        let (_, value) = BinaryOp::parse("+").unwrap();
        assert_eq!(value, BinaryOp::Addition);

        let (_, value) = BinaryOp::parse("-").unwrap();
        assert_eq!(value, BinaryOp::Subtraction);
    }
}
