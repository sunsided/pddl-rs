//! Provides parsers for predicates.

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::DurativeActionSymbol;
use nom::combinator::map;

/// Parses a durative action symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_da_symbol, preamble::*};
/// assert!(parse_da_symbol("abcde").is_value("abcde".into()));
///```
pub fn parse_da_symbol<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, DurativeActionSymbol<'a>> {
    map(parse_name, DurativeActionSymbol::from)(input.into())
}

impl<'a> crate::parsers::Parser<'a> for DurativeActionSymbol<'a> {
    type Item = DurativeActionSymbol<'a>;

    /// See [`parse_da_symbol`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_da_symbol(input)
    }
}
