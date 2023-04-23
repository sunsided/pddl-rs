//! Provides parsers for predicates.

use crate::parsers::utility::parse_name;
use crate::types::DurativeActionSymbol;
use nom::combinator::map;
use nom::IResult;

/// Parses a durative action symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_da_symbol;
/// assert_eq!(parse_da_symbol("abcde"), Ok(("", "abcde".into())));
///```
pub fn parse_da_symbol(input: &str) -> IResult<&str, DurativeActionSymbol> {
    map(parse_name, DurativeActionSymbol::from)(input)
}
