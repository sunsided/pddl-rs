//! Provides parsers for variables.

use crate::parsers::name::parse_name;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;

/// Parses a variable, i.e. `?<name>` and returns its name.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_variable;
/// assert_eq!(parse_variable("?abcde"), Ok(("", "abcde")));
/// assert_eq!(parse_variable("?a-1_2"), Ok(("", "a-1_2")));
/// assert_eq!(parse_variable("?Z01"), Ok(("", "Z01")));
/// assert_eq!(parse_variable("?x-_-_"), Ok(("", "x-_-_")));
///
/// assert!(parse_variable("abcde").is_err());
/// assert!(parse_variable("?-").is_err());
/// assert!(parse_variable("?1").is_err());
///```
pub fn parse_variable(input: &str) -> IResult<&str, &str> {
    preceded(tag("?"), parse_name)(input)
}
