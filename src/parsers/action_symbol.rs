//! Provides parsers for action symbols.

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::ActionSymbol;
use nom::combinator::map;

/// Parses a function symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_action_symbol, Span, UnwrapValue};
/// assert!(parse_action_symbol(Span::new("abcde")).is_value("abcde".into()));
/// assert!(parse_action_symbol(Span::new("a-1_2")).is_value("a-1_2".into()));
/// assert!(parse_action_symbol(Span::new("Z01")).is_value("Z01".into()));
/// assert!(parse_action_symbol(Span::new("x-_-_")).is_value("x-_-_".into()));
///
/// assert!(parse_action_symbol(Span::new("")).is_err());
/// assert!(parse_action_symbol(Span::new(".")).is_err());
/// assert!(parse_action_symbol(Span::new("-abc")).is_err());
/// assert!(parse_action_symbol(Span::new("0124")).is_err());
/// assert!(parse_action_symbol(Span::new("-1")).is_err());
///```
pub fn parse_action_symbol<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, ActionSymbol<'a>> {
    map(parse_name, ActionSymbol::new)(input.into())
}

impl<'a> crate::parsers::Parser<'a> for ActionSymbol<'a> {
    type Item = ActionSymbol<'a>;

    /// See [`parse_action_symbol`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_action_symbol(input.into())
    }
}
