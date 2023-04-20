//! Provides parsers for predicates.

use crate::parsers::name::parse_name;
use crate::types::DASymbol;
use nom::combinator::map;
use nom::IResult;

/// Parses a durative action symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_da_symbol;
/// assert_eq!(parse_da_symbol("abcde"), Ok(("", "abcde".into())));
///```
pub fn parse_da_symbol(input: &str) -> IResult<&str, DASymbol> {
    map(parse_name, DASymbol::from)(input)
}
