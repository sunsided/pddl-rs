//! Provides parsers for names.

use crate::parsers::{ws, ParseResult, Span};
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
/// assert!(parse_name("abcde").is_value("abcde".into()));
/// assert!(parse_name("a-1_2").is_value("a-1_2".into()));
/// assert!(parse_name("Z01").is_value("Z01".into()));
/// assert!(parse_name("x-_-_").is_value("x-_-_".into()));
///
/// assert!(parse_name("").is_err());
/// assert!(parse_name(".").is_err());
/// assert!(parse_name("-abc").is_err());
/// assert!(parse_name("0124").is_err());
/// assert!(parse_name("-1").is_err());
///```
pub fn parse_name<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Name> {
    map(
        ws(recognize(tuple((alpha1, many0(parse_any_char))))),
        |x: Span| Name::from(*x.fragment()),
    )(input.into())
}

/// Parses any accepted character.
pub fn parse_any_char(input: Span) -> ParseResult<Span> {
    recognize(alt((alpha1, digit1, tag("-"), tag("_"))))(input)
}

impl crate::parsers::Parser for Name {
    type Item = Name;

    /// See [`parse_name`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
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
