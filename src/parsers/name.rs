//! Provides parsers for names.

use crate::parsers::{ParseResult, Span};
use crate::types::Name;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1};
use nom::combinator::{map, recognize};
use nom::multi::many0;
use nom::sequence::tuple;

/// Parses a name, i.e. `<letter> <any char>⁺`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_name, preamble::*};
/// assert!(parse_name(Span::new("abcde")).is_value("abcde".into()));
/// assert!(parse_name(Span::new("a-1_2")).is_value("a-1_2".into()));
/// assert!(parse_name(Span::new("Z01")).is_value("Z01".into()));
/// assert!(parse_name(Span::new("x-_-_")).is_value("x-_-_".into()));
///
/// assert!(parse_name(Span::new("")).is_err());
/// assert!(parse_name(Span::new(".")).is_err());
/// assert!(parse_name(Span::new("-abc")).is_err());
/// assert!(parse_name(Span::new("0124")).is_err());
/// assert!(parse_name(Span::new("-1")).is_err());
///```
pub fn parse_name(input: Span) -> ParseResult<Name> {
    map(
        recognize(tuple((alpha1, many0(parse_any_char)))),
        |x: Span| Name::from(*x.fragment()),
    )(input)
}

/// Parses any accepted character.
pub fn parse_any_char(input: Span) -> ParseResult<Span> {
    recognize(alt((alpha1, digit1, tag("-"), tag("_"))))(input)
}

impl<'a> crate::parsers::Parser<'a> for Name<'a> {
    type Item = Name<'a>;

    /// See [`parse_name`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_name(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parsers::Match;

    #[test]
    fn parse_name_works() {
        assert!(parse_name(Span::new("abcde")).is_exactly("abcde"));
    }

    #[test]
    fn parse_any_char_works() {
        assert!(parse_any_char(Span::new("abc")).is_exactly("abc"));
        assert!(parse_any_char(Span::new("1")).is_exactly("1"));
        assert!(parse_any_char(Span::new("-.")).is_result(".", "-"));
        assert!(parse_any_char(Span::new("_")).is_exactly("_"));
        assert!(parse_any_char(Span::new(".")).is_err());
        assert!(parse_any_char(Span::new(".")).is_err());
    }
}
