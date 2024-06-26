//! Provides parsers for function symbols.

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::FunctionSymbol;
use nom::combinator::map;

/// Parses a function symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_function_symbol, preamble::*};
/// assert!(parse_function_symbol("abcde").is_value("abcde".into()));
/// assert!(parse_function_symbol("a-1_2").is_value("a-1_2".into()));
/// assert!(parse_function_symbol("Z01").is_value("Z01".into()));
/// assert!(parse_function_symbol("x-_-_").is_value("x-_-_".into()));
///
/// assert!(parse_function_symbol("").is_err());
/// assert!(parse_function_symbol(".").is_err());
/// assert!(parse_function_symbol("-abc").is_err());
/// assert!(parse_function_symbol("0124").is_err());
/// assert!(parse_function_symbol("-1").is_err());
///```
pub fn parse_function_symbol<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FunctionSymbol> {
    map(parse_name, FunctionSymbol::new)(input.into())
}

impl crate::parsers::Parser for FunctionSymbol {
    type Item = FunctionSymbol;

    /// Parses a function symbol.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{FunctionSymbol, Parser};
    /// let (_, value) = FunctionSymbol::parse("abcde").unwrap();
    /// assert_eq!(value, "abcde".into());
    ///```
    ///
    /// ## See also
    /// See [`parse_function_symbol`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_function_symbol(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{FunctionSymbol, Parser};

    #[test]
    fn test_parse() {
        assert!(FunctionSymbol::parse("abcde").is_value("abcde".into()));
        assert!(FunctionSymbol::parse("a-1_2").is_value("a-1_2".into()));
        assert!(FunctionSymbol::parse("Z01").is_value("Z01".into()));
        assert!(FunctionSymbol::parse("x-_-_").is_value("x-_-_".into()));

        assert!(FunctionSymbol::parse("").is_err());
        assert!(FunctionSymbol::parse(".").is_err());
        assert!(FunctionSymbol::parse("-abc").is_err());
        assert!(FunctionSymbol::parse("0124").is_err());
        assert!(FunctionSymbol::parse("-1").is_err());
    }
}
