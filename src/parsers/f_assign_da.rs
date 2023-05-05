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
/// assert!(parse_f_assign_da("(assign fun-sym ?duration)").is_ok());
///```
pub fn parse_f_assign_da<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FAssignDa> {
    map(
        parens(tuple((
            parse_assign_op,
            preceded(multispace1, parse_f_head),
            preceded(multispace1, parse_f_exp_da),
        ))),
        FAssignDa::from,
    )(input.into())
}

impl crate::parsers::Parser for FAssignDa {
    type Item = FAssignDa;

    /// See [`parse_f_assign_da`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
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
