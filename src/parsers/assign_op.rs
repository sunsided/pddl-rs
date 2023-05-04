//! Provides parsers for assignment operations.

use crate::parsers::{ParseResult, Span};
use crate::types::assign_op::names;
use crate::types::AssignOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses an assignment operation, i.e. `assign | scale-up | scale-down | increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_assign_op, Span, UnwrapValue};
/// # use pddl::{AssignOp};
/// assert!(parse_assign_op(Span::new("assign")).is_value(AssignOp::Assign));
/// assert!(parse_assign_op(Span::new("scale-up")).is_value(AssignOp::ScaleUp));
///```
pub fn parse_assign_op(input: Span) -> ParseResult<AssignOp> {
    map(
        alt((
            tag(names::CHANGE), // deprecated
            tag(names::ASSIGN),
            tag(names::SCALE_UP),
            tag(names::SCALE_DOWN),
            tag(names::INCREASE),
            tag(names::DECREASE),
        )),
        |x: Span| AssignOp::try_from(*x.fragment()).expect("unhandled variant"),
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for AssignOp {
    type Item = AssignOp;

    /// See [`parse_assign_op`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_assign_op(input)
    }
}
