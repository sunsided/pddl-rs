//! Provides parsers for assignment operations.

use crate::types::time_specifier::names;
use crate::types::TimeSpecifier;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_time_specifier;
/// # use pddl::{TimeSpecifier};
/// assert_eq!(parse_time_specifier("start"), Ok(("", TimeSpecifier::Start)));
/// assert_eq!(parse_time_specifier("end"), Ok(("", TimeSpecifier::End)));
///```
pub fn parse_time_specifier(input: &str) -> IResult<&str, TimeSpecifier> {
    map_res(
        alt((tag(names::START), tag(names::END))),
        TimeSpecifier::try_from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for TimeSpecifier {
    type Item = TimeSpecifier;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_time_specifier(input)
    }
}
