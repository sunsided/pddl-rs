//! Provides parsers for function symbols.

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::FunctionSymbol;

/// Parses a function symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_function_symbol, preamble::*};
/// assert!(parse_function_symbol("abcde".into()).is_value("abcde".into()));
/// assert!(parse_function_symbol("a-1_2".into()).is_value("a-1_2".into()));
/// assert!(parse_function_symbol("Z01".into()).is_value("Z01".into()));
/// assert!(parse_function_symbol("x-_-_".into()).is_value("x-_-_".into()));
///
/// assert!(parse_function_symbol("".into()).is_err());
/// assert!(parse_function_symbol(".".into()).is_err());
/// assert!(parse_function_symbol("-abc".into()).is_err());
/// assert!(parse_function_symbol("0124".into()).is_err());
/// assert!(parse_function_symbol("-1".into()).is_err());
///```
pub fn parse_function_symbol(input: Span) -> ParseResult<FunctionSymbol> {
    let (remaining, name) = parse_name(input)?;
    Ok((remaining, name.into()))
}

impl<'a> crate::parsers::Parser<'a> for FunctionSymbol<'a> {
    type Item = FunctionSymbol<'a>;

    /// See [`parse_function_symbol`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_function_symbol(input.into())
    }
}
