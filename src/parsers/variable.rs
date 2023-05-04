//! Provides parsers for variables.

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::Variable;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;

/// Parses a variable, i.e. `?<name>` and returns its name.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_variable, preamble::*};
/// assert!(parse_variable(Span::new("?abcde")).is_value("abcde".into()));
/// assert!(parse_variable(Span::new("?a-1_2")).is_value("a-1_2".into()));
/// assert!(parse_variable(Span::new("?Z01")).is_value("Z01".into()));
/// assert!(parse_variable(Span::new("?x-_-_")).is_value("x-_-_".into()));
///
/// assert!(parse_variable(Span::new("abcde")).is_err());
/// assert!(parse_variable(Span::new("?-")).is_err());
/// assert!(parse_variable(Span::new("?1")).is_err());
///```
pub fn parse_variable<'a>(input: Span<'a>) -> ParseResult<'a, Variable> {
    map(preceded(tag("?"), parse_name), Variable::from)(input)
}

impl<'a> crate::parsers::Parser<'a> for Variable<'a> {
    type Item = Variable<'a>;

    /// See [`parse_variable`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_variable(input.into())
    }
}
