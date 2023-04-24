//! Provides parsers for assignment operations.

use crate::types::assign_op_t::names;
use crate::types::AssignOpT;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_assign_op_t;
/// # use pddl::types::{AssignOpT};
/// assert_eq!(parse_assign_op_t("increase"), Ok(("", AssignOpT::Increase)));
/// assert_eq!(parse_assign_op_t("decrease"), Ok(("", AssignOpT::Decrease)));
///```
pub fn parse_assign_op_t(input: &str) -> IResult<&str, AssignOpT> {
    map_res(
        alt((tag(names::INCREASE), tag(names::DECREASE))),
        AssignOpT::try_from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for AssignOpT {
    type Item = AssignOpT;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_assign_op_t(input)
    }
}
