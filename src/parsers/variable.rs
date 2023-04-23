//! Provides parsers for variables.

use crate::parsers::parse_name;
use crate::types::Variable;
use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::IResult;

/// Parses a variable, i.e. `?<name>` and returns its name.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_variable;
/// assert_eq!(parse_variable("?abcde"), Ok(("", "abcde".into())));
/// assert_eq!(parse_variable("?a-1_2"), Ok(("", "a-1_2".into())));
/// assert_eq!(parse_variable("?Z01"), Ok(("", "Z01".into())));
/// assert_eq!(parse_variable("?x-_-_"), Ok(("", "x-_-_".into())));
///
/// assert!(parse_variable("abcde").is_err());
/// assert!(parse_variable("?-").is_err());
/// assert!(parse_variable("?1").is_err());
///```
pub fn parse_variable(input: &str) -> IResult<&str, Variable> {
    let (remaining, name) = preceded(tag("?"), parse_name)(input)?;
    Ok((remaining, Variable::from(name)))
}
