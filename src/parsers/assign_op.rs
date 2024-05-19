//! Provides parsers for assignment operations.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

use crate::parsers::{ParseResult, Span};
use crate::types::assign_op::names;
use crate::types::AssignOp;

/// Parses an assignment operation, i.e. `assign | scale-up | scale-down | increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_assign_op, Span, UnwrapValue};
/// # use pddl::{AssignOp};
/// assert!(parse_assign_op(Span::new("assign")).is_value(AssignOp::Assign));
/// assert!(parse_assign_op(Span::new("scale-up")).is_value(AssignOp::ScaleUp));
///```
pub fn parse_assign_op<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, AssignOp> {
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
    )(input.into())
}

impl crate::parsers::Parser for AssignOp {
    type Item = AssignOp;

    /// Parses an assignment operation.
    ///
    /// ## Example
    /// ```
    /// # use pddl::parsers::{ Span, UnwrapValue};
    /// # use pddl::{AssignOp, Parser};
    /// let (_, assign_op) = AssignOp::parse("assign").unwrap();
    /// assert_eq!(assign_op, AssignOp::Assign);
    ///
    /// let (_, assign_op) = AssignOp::parse("scale-up").unwrap();
    /// assert_eq!(assign_op, AssignOp::ScaleUp);
    ///```
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_assign_op(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AssignOp, Parser};

    #[test]
    fn test_parse() {
        let (_, assign_op) = AssignOp::parse("assign").unwrap();
        assert_eq!(assign_op, AssignOp::Assign);

        let (_, assign_op) = AssignOp::parse("scale-up").unwrap();
        assert_eq!(assign_op, AssignOp::ScaleUp);
    }
}
