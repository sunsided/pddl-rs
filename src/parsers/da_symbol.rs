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
pub fn parse_da_symbol<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, DurativeActionSymbol> {
    map(parse_name, DurativeActionSymbol::from)(input.into())
}

impl crate::parsers::Parser for DurativeActionSymbol {
    type Item = DurativeActionSymbol;

    /// Parses a durative action symbol.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{DurativeActionSymbol, Parser};
    /// let (_, value) = DurativeActionSymbol::parse("abcde").unwrap();
    /// assert_eq!(value, "abcde".into());
    ///```
    ///
    /// ## See also
    /// See [`parse_da_symbol`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_da_symbol(input)
    }
}
