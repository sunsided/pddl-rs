//! Provides parsers for assignment operations.

use crate::types::interval::names;
use crate::types::Interval;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an intervals, i.e. `all`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_interval;
/// # use pddl::types::{Interval};
/// assert_eq!(parse_interval("all"), Ok(("", Interval::All)));
///```
pub fn parse_interval(input: &str) -> IResult<&str, Interval> {
    map_res(tag(names::ALL), Interval::try_from)(input)
}