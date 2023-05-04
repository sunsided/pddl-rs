//! Provides parsers for f-assign-das.

use crate::parsers::{parens, ParseResult, Span};
use crate::parsers::{parse_assign_op, parse_f_exp_da, parse_f_head};
use crate::types::FAssignDa;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses an f-assign-da.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_f_assign_da;
/// assert!(parse_f_assign_da("(assign fun-sym ?duration)".into()).is_ok());
///```
pub fn parse_f_assign_da(input: Span) -> ParseResult<FAssignDa> {
    map(
        parens(tuple((
            parse_assign_op,
            preceded(multispace1, parse_f_head),
            preceded(multispace1, parse_f_exp_da),
        ))),
        FAssignDa::from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for FAssignDa<'a> {
    type Item = FAssignDa<'a>;

    /// See [`parse_f_assign_da`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_f_assign_da(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "(increase (distance-travelled) 5)";
        let (_, _effect) = parse_f_assign_da(Span::new(input)).unwrap();
    }
}
