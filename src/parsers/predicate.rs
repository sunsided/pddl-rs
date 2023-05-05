//! Provides parsers for predicates.

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::Predicate;
use nom::combinator::map;

/// Parses a predicate, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_predicate, preamble::*};
/// assert!(parse_predicate(Span::new("abcde")).is_value("abcde".into()));
/// assert!(parse_predicate(Span::new("a-1_2")).is_value("a-1_2".into()));
/// assert!(parse_predicate(Span::new("Z01")).is_value("Z01".into()));
/// assert!(parse_predicate(Span::new("x-_-_")).is_value("x-_-_".into()));
///
/// assert!(parse_predicate(Span::new("")).is_err());
/// assert!(parse_predicate(Span::new(".")).is_err());
/// assert!(parse_predicate(Span::new("-abc")).is_err());
/// assert!(parse_predicate(Span::new("0124")).is_err());
/// assert!(parse_predicate(Span::new("-1")).is_err());
///```
pub fn parse_predicate<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Predicate<'a>> {
    map(parse_name, Predicate::from)(input.into())
}

impl<'a> crate::parsers::Parser<'a> for Predicate<'a> {
    type Item = Predicate<'a>;

    /// See [`parse_predicate`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_predicate(input)
    }
}
