//! Provides parsers for binary-operand operations.

use crate::types::{binary_op::names, BinaryOp};
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
        alt((
            tag(names::MULTIPLICATION),
            tag(names::ADDITION),
            tag(names::SUBTRACTION),
            tag(names::DIVISION),
        )),
        BinaryOp::try_from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for BinaryOp {
    type Item = BinaryOp;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_binary_op(input)
    }
}
