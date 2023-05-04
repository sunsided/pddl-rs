//! Provides parsers for preference names.
use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::PreferenceName;
use nom::combinator::map;

/// Parses a preference name.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_pref_name, preamble::*};
/// assert!(parse_pref_name("abcde".into()).is_value("abcde".into()));
///```
pub fn parse_pref_name(input: Span) -> ParseResult<PreferenceName> {
    map(parse_name, PreferenceName::new)(input)
}

impl<'a> crate::parsers::Parser<'a> for PreferenceName<'a> {
    type Item = PreferenceName<'a>;

    /// See [`parse_pref_name`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_pref_name(input)
    }
}
