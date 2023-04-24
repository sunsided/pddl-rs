//! Provides parsers for f-comps.

use crate::parsers::parens;
use crate::parsers::{parse_binary_comp, parse_f_exp};
use crate::types::FComp;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses an f-comp.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_f_comp;
/// # use pddl::types::{FunctionTerm, Variable, FunctionSymbol, Term, FComp, BinaryComp, FExp, BinaryOp};
/// assert_eq!(parse_f_comp("(= (+ 1.23 2.34) (+ 1.23 2.34))"), Ok(("",
///     FComp::new(
///         BinaryComp::Equal,
///         FExp::new_binary_op(
///             BinaryOp::Addition,
///             FExp::new_number(1.23),
///             FExp::new_number(2.34),
///         ),
///         FExp::new_binary_op(
///             BinaryOp::Addition,
///             FExp::new_number(1.23),
///             FExp::new_number(2.34),
///         )
///     )
/// )));
///```
pub fn parse_f_comp(input: &str) -> IResult<&str, FComp> {
    map(
        parens(tuple((
            parse_binary_comp,
            preceded(multispace1, parse_f_exp),
            preceded(multispace1, parse_f_exp),
        ))),
        FComp::from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for FComp<'a> {
    type Item = FComp<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_f_comp(input)
    }
}
