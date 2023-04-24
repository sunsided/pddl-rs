//! Provides parsers for predicates.

use crate::parsers::parse_name;
use crate::types::DurativeActionSymbol;
use nom::combinator::map;
use nom::IResult;

/// Parses a durative action symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_da_symbol;
/// assert_eq!(parse_da_symbol("abcde"), Ok(("", "abcde".into())));
///```
pub fn parse_da_symbol(input: &str) -> IResult<&str, DurativeActionSymbol> {
    map(parse_name, DurativeActionSymbol::from)(input)
}

impl<'a> crate::parsers::Parser<'a> for DurativeActionSymbol<'a> {
    type Item = DurativeActionSymbol<'a>;

    /// See [`parse_da_symbol`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_da_symbol(input)
    }
}
