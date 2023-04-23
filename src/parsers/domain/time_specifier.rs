//! Provides parsers for assignment operations.

use crate::types::domain::time_specifier::names;
use crate::types::domain::TimeSpecifier;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_time_specifier;
/// # use pddl::types::domain::{TimeSpecifier};
/// assert_eq!(parse_time_specifier("start"), Ok(("", TimeSpecifier::Start)));
/// assert_eq!(parse_time_specifier("end"), Ok(("", TimeSpecifier::End)));
///```
pub fn parse_time_specifier(input: &str) -> IResult<&str, TimeSpecifier> {
    map_res(
        alt((tag(names::START), tag(names::END))),
        TimeSpecifier::try_from,
    )(input)
}