//! Provides parsers for assignment operations.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::Parser;

use crate::parsers::{ParseResult, Span};
use crate::types::assign_op_t::names;
use crate::types::TimedAssignOperator;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_assign_op_t, Span, UnwrapValue};
/// # use pddl::{TimedAssignOperator};
/// assert!(parse_assign_op_t(Span::new("increase")).is_value(TimedAssignOperator::Increase));
/// assert!(parse_assign_op_t(Span::new("decrease")).is_value(TimedAssignOperator::Decrease));
///```
pub fn parse_assign_op_t<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, TimedAssignOperator> {
    map(
        alt((tag(names::INCREASE), tag(names::DECREASE))),
        |x: Span| TimedAssignOperator::try_from(*x.fragment()).expect("unhandled variant"),
    )
    .parse(input.into())
}

impl crate::parsers::Parser for TimedAssignOperator {
    type Item = TimedAssignOperator;

    /// Parses an assignment operation.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{TimedAssignOperator, Parser};
    /// let (_, value) = TimedAssignOperator::parse("increase").unwrap();
    /// assert_eq!(value, TimedAssignOperator::Increase);
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
    use crate::{Parser, TimedAssignOperator};

    #[test]
    fn test_parse() {
        let (_, value) = TimedAssignOperator::parse("increase").unwrap();
        assert_eq!(value, TimedAssignOperator::Increase);
    }
}
