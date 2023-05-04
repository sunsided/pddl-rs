//! Provides parsers for predicates.

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::DurativeActionSymbol;
use nom::combinator::map;

/// Parses a durative action symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_da_symbol, preamble::*};
/// assert!(parse_da_symbol("abcde".into()).is_value("abcde".into()));
///```
pub fn parse_da_symbol(input: Span) -> ParseResult<DurativeActionSymbol> {
    map(parse_name, DurativeActionSymbol::from)(input)
}

impl<'a> crate::parsers::Parser<'a> for DurativeActionSymbol<'a> {
    type Item = DurativeActionSymbol<'a>;

    /// See [`parse_da_symbol`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_da_symbol(input)
    }
}
