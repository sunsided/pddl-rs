//! Provides parsers for binary comparison operations.

use crate::parsers::{ParseResult, Span};
use crate::types::{binary_comp::names, BinaryComp};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses an assignment operation, i.e. `assign | scale-up | scale-down | increase | decrease`.
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
pub fn parse_binary_comp(input: Span) -> ParseResult<BinaryComp> {
    map(
        alt((
            tag(names::GREATER_THAN_OR_EQUAL),
            tag(names::LESS_THAN_OR_EQUAL),
            tag(names::EQUAL),
            tag(names::GREATER_THAN),
            tag(names::LESS_THAN),
        )),
        |x: Span| BinaryComp::try_from(*x.fragment()).expect("unhandled variant"),
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for BinaryComp {
    type Item = BinaryComp;

    /// See [`parse_binary_comp`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_binary_comp(input)
    }
}
