//! Provides parsers for binary comparison operations.

use crate::types::BinaryComp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an assignment operation, i.e. `assign | scale-up | scale-down | increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_binary_comp;
/// # use pddl::types::{AssignOp, BinaryComp};
/// assert_eq!(parse_binary_comp(">"), Ok(("", BinaryComp::GreaterThan)));
/// assert_eq!(parse_binary_comp("<"), Ok(("", BinaryComp::LessThan)));
/// assert_eq!(parse_binary_comp("="), Ok(("", BinaryComp::Equal)));
/// assert_eq!(parse_binary_comp(">="), Ok(("", BinaryComp::GreaterOrEqual)));
/// assert_eq!(parse_binary_comp("<="), Ok(("", BinaryComp::LessThanOrEqual)));
///```
pub fn parse_binary_comp(input: &str) -> IResult<&str, BinaryComp> {
    map_res(
        alt((tag("<="), tag(">="), tag("="), tag(">"), tag("<"))),
        BinaryComp::try_from,
    )(input)
}
