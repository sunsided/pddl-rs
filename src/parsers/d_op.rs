//! Provides parsers for durative operations.

use crate::types::{d_op::names, DOp};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses a durative operation, i.e. `<= | >= | =`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_d_op;
/// # use pddl::{DOp};
/// assert_eq!(parse_d_op("<="), Ok(("", DOp::LessThanOrEqual)));
/// assert_eq!(parse_d_op(">="), Ok(("", DOp::GreaterOrEqual)));
/// assert_eq!(parse_d_op("="), Ok(("", DOp::Equal)));
///```
pub fn parse_d_op(input: &str) -> IResult<&str, DOp> {
    // :duration-inequalities
    map_res(
        alt((
            tag(names::LESS_THAN_OR_EQUAL),
            tag(names::GREATER_OR_EQUAL),
            tag(names::EQUAL),
        )),
        DOp::try_from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for DOp {
    type Item = DOp;

    /// See [`parse_d_op`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_d_op(input)
    }
}
