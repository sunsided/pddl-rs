//! Provides parsers for function symbols.

use crate::parsers::name::parse_name;
use nom::IResult;

/// Parses a function symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_function_symbol;
/// assert_eq!(parse_function_symbol("abcde"), Ok(("", "abcde")));
/// assert_eq!(parse_function_symbol("a-1_2"), Ok(("", "a-1_2")));
/// assert_eq!(parse_function_symbol("Z01"), Ok(("", "Z01")));
/// assert_eq!(parse_function_symbol("x-_-_"), Ok(("", "x-_-_")));
///
/// assert!(parse_function_symbol("").is_err());
/// assert!(parse_function_symbol(".").is_err());
/// assert!(parse_function_symbol("-abc").is_err());
/// assert!(parse_function_symbol("0124").is_err());
/// assert!(parse_function_symbol("-1").is_err());
///```
pub fn parse_function_symbol(input: &str) -> IResult<&str, &str> {
    parse_name(input)
}
