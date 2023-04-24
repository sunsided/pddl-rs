//! Provides parsers for binary comparison operations.

use crate::types::{binary_comp::names, BinaryComp};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an assignment operation, i.e. `assign | scale-up | scale-down | increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_binary_comp;
/// # use pddl::{AssignOp, BinaryComp};
/// assert_eq!(parse_binary_comp(">"), Ok(("", BinaryComp::GreaterThan)));
/// assert_eq!(parse_binary_comp("<"), Ok(("", BinaryComp::LessThan)));
/// assert_eq!(parse_binary_comp("="), Ok(("", BinaryComp::Equal)));
/// assert_eq!(parse_binary_comp(">="), Ok(("", BinaryComp::GreaterOrEqual)));
/// assert_eq!(parse_binary_comp("<="), Ok(("", BinaryComp::LessThanOrEqual)));
///```
pub fn parse_binary_comp(input: &str) -> IResult<&str, BinaryComp> {
    map_res(
        alt((
            tag(names::GREATER_THAN_OR_EQUAL),
            tag(names::LESS_THAN_OR_EQUAL),
            tag(names::EQUAL),
            tag(names::GREATER_THAN),
            tag(names::LESS_THAN),
        )),
        BinaryComp::try_from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for BinaryComp {
    type Item = BinaryComp;

    /// See [`parse_binary_comp`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_binary_comp(input)
    }
}
