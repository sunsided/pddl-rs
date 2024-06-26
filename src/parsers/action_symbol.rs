//! Provides parsers for action symbols.

use nom::combinator::map;

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::ActionSymbol;

/// Parses an action symbol, i.e. `<name>`.
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
pub fn parse_action_symbol<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, ActionSymbol> {
    map(parse_name, ActionSymbol::new)(input.into())
}

impl crate::parsers::Parser for ActionSymbol {
    type Item = ActionSymbol;

    /// Parses an action symbol.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{ActionSymbol, Parser};
    /// let (_, action_symbol) = ActionSymbol::parse("abcde").unwrap();
    /// assert_eq!(action_symbol, "abcde".into());
    ///```
    ///
    /// ## See also
    /// See [`parse_action_symbol`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_action_symbol(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ActionSymbol, Parser};

    #[test]
    fn test_parse() {
        let (_, action_symbol) = ActionSymbol::parse("abcde").unwrap();
        assert_eq!(action_symbol, "abcde".into());
    }
}
