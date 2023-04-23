//! Provides parsers for optimization.

use crate::types::{optimization::names, Optimization};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an optimization goal, i.e. `minimize | maximize`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_optimization;
/// # use pddl::types::{Optimization};
/// assert_eq!(parse_optimization("minimize"), Ok(("", Optimization::Minimize)));
/// assert_eq!(parse_optimization("maximize"), Ok(("", Optimization::Maximize)));
///```
pub fn parse_optimization(input: &str) -> IResult<&str, Optimization> {
    map_res(
        alt((tag(names::MINIMIZE), tag(names::MAXIMIZE))),
        Optimization::try_from,
    )(input)
}
