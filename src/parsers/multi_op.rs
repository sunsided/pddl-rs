//! Provides parsers for multi-operand operations.

use crate::parsers::{ParseResult, Span};
use crate::types::{multi_op::names, MultiOp};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses a multi-operand operation, i.e. `* | +`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_multi_op, preamble::*};
/// # use pddl::{MultiOp};
/// assert!(parse_multi_op("*").is_value(MultiOp::Multiplication));
/// assert!(parse_multi_op("+").is_value(MultiOp::Addition));
///```
pub fn parse_multi_op<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, MultiOp> {
    map(
        alt((tag(names::MULTIPLICATION), tag(names::ADDITION))),
        |x: Span| MultiOp::try_from(*x.fragment()).expect("unhandled variant"),
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for MultiOp {
    type Item = MultiOp;

    /// See [`parse_multi_op`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_multi_op(input.into())
    }
}
