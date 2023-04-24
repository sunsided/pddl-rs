//! Provides parsers for assignment operations.

use crate::types::assign_op::names;
use crate::types::AssignOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an assignment operation, i.e. `assign | scale-up | scale-down | increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_assign_op;
/// # use pddl::types::{AssignOp};
/// assert_eq!(parse_assign_op("assign"), Ok(("", AssignOp::Assign)));
/// assert_eq!(parse_assign_op("scale-up"), Ok(("", AssignOp::ScaleUp)));
///```
pub fn parse_assign_op(input: &str) -> IResult<&str, AssignOp> {
    map_res(
        alt((
            tag(names::ASSIGN),
            tag(names::SCALE_UP),
            tag(names::SCALE_DOWN),
            tag(names::INCREASE),
            tag(names::DECREASE),
        )),
        AssignOp::try_from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for AssignOp {
    type Item = AssignOp;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_assign_op(input)
    }
}
