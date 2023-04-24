//! Provides parsers for f-exp-da values..

use crate::parsers::{parens, space_separated_list1};
use crate::parsers::{parse_binary_op, parse_f_exp, parse_multi_op};
use crate::types::FExpDa;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses an f-exp-da.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_f_exp_da;
/// # use pddl::{BinaryOp, FExpDa, FExp, FunctionSymbol, MultiOp};
/// assert_eq!(parse_f_exp_da("?duration"), Ok(("",
///     FExpDa::Duration
/// )));
///
/// assert_eq!(parse_f_exp_da("(+ 1.23 2.34)"), Ok(("",
///     FExpDa::new_binary_op(
///         BinaryOp::Addition,
///             FExpDa::new_f_exp(FExp::new_number(1.23)),
///             FExpDa::new_f_exp(FExp::new_number(2.34))
///     )
/// )));
///
/// assert_eq!(parse_f_exp_da("(+ 1.23 2.34 3.45)"), Ok(("",
///     FExpDa::new_multi_op(
///         MultiOp::Addition,
///         FExpDa::new_f_exp(FExp::new_number(1.23)),
///         [FExp::new_number(2.34).into(), FExp::new_number(3.45).into()]
///     )
/// )));
///```
pub fn parse_f_exp_da(input: &str) -> IResult<&str, FExpDa> {
    // :duration-inequalities
    let duration = map(tag("?duration"), |_| FExpDa::new_duration());

    let binary_op = map(
        parens(tuple((
            parse_binary_op,
            preceded(multispace1, parse_f_exp_da),
            preceded(multispace1, parse_f_exp_da),
        ))),
        |(op, lhs, rhs)| FExpDa::new_binary_op(op, lhs, rhs),
    );

    let multi_op = map(
        parens(tuple((
            parse_multi_op,
            preceded(multispace1, parse_f_exp_da),
            preceded(multispace1, space_separated_list1(parse_f_exp_da)),
        ))),
        |(op, lhs, rhs)| FExpDa::new_multi_op(op, lhs, rhs),
    );

    let negated = map(
        parens(preceded(tuple((char('-'), multispace0)), parse_f_exp_da)),
        FExpDa::new_negative,
    );

    let f_exp = map(parse_f_exp, FExpDa::new_f_exp);

    alt((duration, binary_op, multi_op, negated, f_exp))(input)
}

impl<'a> crate::parsers::Parser<'a> for FExpDa<'a> {
    type Item = FExpDa<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_f_exp_da(input)
    }
}
