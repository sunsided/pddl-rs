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
pub fn parse_predicate<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Predicate> {
    map(parse_name, Predicate::from)(input.into())
}

impl crate::parsers::Parser for Predicate {
    type Item = Predicate;

    /// Parses a predicate.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Predicate, Parser};
    /// let (_, value) = Predicate::parse("abcde").unwrap();
    /// assert_eq!(value, "abcde".into());
    ///```
    ///
    /// See [`parse_predicate`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_predicate(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Parser, Predicate};

    #[test]
    fn test_parse() {
        let (_, value) = Predicate::parse("abcde").unwrap();
        assert_eq!(value, "abcde".into());
    }
}
