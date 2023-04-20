//! Provides parsers for f-assign-das.

use crate::parsers::{parens, parse_assign_op, parse_f_exp_da, parse_f_head};
use crate::types::FAssignDa;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses an f-assign-da.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_f_assign_da;
/// assert!(parse_f_assign_da("(assign fun-sym ?duration)").is_ok());
///```
pub fn parse_f_assign_da(input: &str) -> IResult<&str, FAssignDa> {
    map(
        parens(tuple((
            parse_assign_op,
            preceded(multispace1, parse_f_head),
            preceded(multispace1, parse_f_exp_da),
        ))),
        FAssignDa::from,
    )(input)
}