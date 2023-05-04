//! Provides parsers for binary-operand operations.

use crate::parsers::{ParseResult, Span};
use crate::types::{binary_op::names, BinaryOp};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses a multi-operand operation, i.e. `* | + | - | /`.
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
pub fn parse_binary_op(input: Span) -> ParseResult<BinaryOp> {
    map(
        alt((
            tag(names::MULTIPLICATION),
            tag(names::ADDITION),
            tag(names::SUBTRACTION),
            tag(names::DIVISION),
        )),
        |x: Span| BinaryOp::try_from(*x.fragment()).expect("unhandled variant"),
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for BinaryOp {
    type Item = BinaryOp;

    /// See [`parse_binary_op`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_binary_op(input.into())
    }
}
