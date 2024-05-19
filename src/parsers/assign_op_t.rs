//! Provides parsers for assignment operations.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

use crate::parsers::{ParseResult, Span};
use crate::types::assign_op_t::names;
use crate::types::AssignOpT;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_assign_op_t, Span, UnwrapValue};
/// # use pddl::{AssignOpT};
/// assert!(parse_assign_op_t(Span::new("increase")).is_value(AssignOpT::Increase));
/// assert!(parse_assign_op_t(Span::new("decrease")).is_value(AssignOpT::Decrease));
///```
pub fn parse_assign_op_t<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, AssignOpT> {
    map(
        alt((tag(names::INCREASE), tag(names::DECREASE))),
        |x: Span| AssignOpT::try_from(*x.fragment()).expect("unhandled variant"),
    )(input.into())
}

impl crate::parsers::Parser for AssignOpT {
    type Item = AssignOpT;

    /// Parses an assignment operation.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{AssignOpT, Parser};
    /// let (_, value) = AssignOpT::parse("increase").unwrap();
    /// assert_eq!(value, AssignOpT::Increase);
    ///```
    ///
    /// ## See also
    /// See [`parse_assign_op_t`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_assign_op_t(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AssignOpT, Parser};

    #[test]
    fn test_parse() {
        let (_, value) = AssignOpT::parse("increase").unwrap();
        assert_eq!(value, AssignOpT::Increase);
    }
}
