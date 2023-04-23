//! Provides parsers for preference names.
use crate::parsers::parse_name;
use crate::types::PreferenceName;
use nom::combinator::map;
use nom::IResult;

/// Parses a preference name.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_pref_name;
/// assert_eq!(parse_pref_name("abcde"), Ok(("", "abcde".into())));
///```
pub fn parse_pref_name(input: &str) -> IResult<&str, PreferenceName> {
    map(parse_name, PreferenceName::new)(input)
}
