//! Provides parsers for multi-operand operations.

use crate::types::MultiOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses a multi-operand operation, i.e. `* | +`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_multi_op;
/// # use pddl::types::{MultiOp};
/// assert_eq!(parse_multi_op("*"), Ok(("", MultiOp::Multiplication)));
/// assert_eq!(parse_multi_op("+"), Ok(("", MultiOp::Addition)));
///```
pub fn parse_multi_op(input: &str) -> IResult<&str, MultiOp> {
    map_res(alt((tag("*"), tag("+"))), MultiOp::try_from)(input)
}
