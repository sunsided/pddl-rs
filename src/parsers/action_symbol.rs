//! Provides parsers for action symbols.

use crate::parsers::parse_name;
use crate::types::ActionSymbol;
use nom::IResult;

/// Parses a function symbol, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_action_symbol;
/// assert_eq!(parse_action_symbol("abcde"), Ok(("", "abcde".into())));
/// assert_eq!(parse_action_symbol("a-1_2"), Ok(("", "a-1_2".into())));
/// assert_eq!(parse_action_symbol("Z01"), Ok(("", "Z01".into())));
/// assert_eq!(parse_action_symbol("x-_-_"), Ok(("", "x-_-_".into())));
///
/// assert!(parse_action_symbol("").is_err());
/// assert!(parse_action_symbol(".").is_err());
/// assert!(parse_action_symbol("-abc").is_err());
/// assert!(parse_action_symbol("0124").is_err());
/// assert!(parse_action_symbol("-1").is_err());
///```
pub fn parse_action_symbol(input: &str) -> IResult<&str, ActionSymbol> {
    let (remaining, name) = parse_name(input)?;
    Ok((remaining, name.into()))
}
