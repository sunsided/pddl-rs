//! Provides parsers for assignment operations.

use crate::parsers::{ParseResult, Span};
use crate::types::assign_op_t::names;
use crate::types::AssignOpT;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_assign_op_t, Span, UnwrapValue};
/// # use pddl::{AssignOpT};
/// assert!(parse_assign_op_t(Span::new("increase")).is_value(AssignOpT::Increase));
/// assert!(parse_assign_op_t(Span::new("decrease")).is_value(AssignOpT::Decrease));
///```
pub fn parse_assign_op_t(input: Span) -> ParseResult<AssignOpT> {
    map(
        alt((tag(names::INCREASE), tag(names::DECREASE))),
        |x: Span| AssignOpT::try_from(*x.fragment()).expect("unhandled variant"),
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for AssignOpT {
    type Item = AssignOpT;

    /// See [`parse_assign_op_t`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_assign_op_t(input)
    }
}
