//! Provides parsers for durative operations.

use crate::types::DOp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parses a durative operation, i.e. `<= | >= | =`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_d_op;
/// # use pddl::types::{DOp};
/// assert_eq!(parse_d_op("<="), Ok(("", DOp::LessThanOrEqual)));
/// assert_eq!(parse_d_op(">="), Ok(("", DOp::GreaterOrEqual)));
/// assert_eq!(parse_d_op("="), Ok(("", DOp::Equal)));
///```
pub fn parse_d_op(input: &str) -> IResult<&str, DOp> {
    // :duration-inequalities
    map_res(alt((tag("<="), tag(">="), tag("="))), DOp::try_from)(input)
}
