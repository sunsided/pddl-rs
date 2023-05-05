//! Provides parsers for durative operations.

use crate::parsers::{ParseResult, Span};
use crate::types::{d_op::names, DOp};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses a durative operation, i.e. `<= | >= | =`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_d_op, preamble::*};
/// # use pddl::{DOp};
/// assert!(parse_d_op("<=").is_value(DOp::LessThanOrEqual));
/// assert!(parse_d_op(">=").is_value(DOp::GreaterOrEqual));
/// assert!(parse_d_op("=").is_value(DOp::Equal));
///```
pub fn parse_d_op<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, DOp> {
    // :duration-inequalities
    map(
        alt((
            tag(names::LESS_THAN_OR_EQUAL),
            tag(names::GREATER_OR_EQUAL),
            tag(names::EQUAL),
        )),
        |x: Span| DOp::try_from(*x.fragment()).expect("unhandled variant"),
    )(input.into())
}

impl crate::parsers::Parser for DOp {
    type Item = DOp;

    /// See [`parse_d_op`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_d_op(input)
    }
}
