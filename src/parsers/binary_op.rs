//! Provides parsers for binary-operand operations.

use crate::types::BinaryOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses a multi-operand operation, i.e. `* | + | - | /`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_binary_op;
/// # use pddl::types::{BinaryOp};
/// assert_eq!(parse_binary_op("*"), Ok(("", BinaryOp::Multiplication)));
/// assert_eq!(parse_binary_op("+"), Ok(("", BinaryOp::Addition)));
/// assert_eq!(parse_binary_op("-"), Ok(("", BinaryOp::Subtraction)));
/// assert_eq!(parse_binary_op("/"), Ok(("", BinaryOp::Division)));
///```
pub fn parse_binary_op(input: &str) -> IResult<&str, BinaryOp> {
    map_res(
        alt((tag("*"), tag("+"), tag("-"), tag("/"))),
        BinaryOp::try_from,
    )(input)
}
