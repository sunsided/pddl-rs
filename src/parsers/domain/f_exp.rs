//! Provides parsers for f-exps.

use crate::parsers::domain::{
    parens, parse_binary_op, parse_f_head, parse_multi_op, parse_number, space_separated_list1,
};
use crate::types::domain::FExp;
use nom::branch::alt;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses an f-exp.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_f_exp;
/// # use pddl::types::domain::{BinaryOp, FExp, FHead, FunctionSymbol, MultiOp};
/// assert_eq!(parse_f_exp("1.23"), Ok(("",
///     FExp::new_number(1.23)
/// )));
///
/// assert_eq!(parse_f_exp("(+ 1.23 2.34)"), Ok(("",
///     FExp::new_binary_op(
///         BinaryOp::Addition,
///         FExp::new_number(1.23),
///         FExp::new_number(2.34)
///     )
/// )));
///
/// assert_eq!(parse_f_exp("(+ 1.23 2.34 3.45)"), Ok(("",
///     FExp::new_multi_op(
///         MultiOp::Addition,
///         FExp::new_number(1.23),
///         [FExp::new_number(2.34), FExp::new_number(3.45)]
///     )
/// )));
///
/// assert_eq!(parse_f_exp("(- 1.23)"), Ok(("",
///     FExp::new_negative(FExp::new_number(1.23))
/// )));
///
/// assert_eq!(parse_f_exp("fun-sym"), Ok(("",
///     FExp::new_f_head(
///         FHead::new(FunctionSymbol::from_str("fun-sym"))
///     )
/// )));
///```
pub fn parse_f_exp(input: &str) -> IResult<&str, FExp> {
    // :numeric-fluents
    let number = map(parse_number, FExp::new_number);

    // :numeric-fluents
    let binary_op = map(
        parens(tuple((
            parse_binary_op,
            preceded(multispace1, parse_f_exp),
            preceded(multispace1, parse_f_exp),
        ))),
        |(op, lhs, rhs)| FExp::new_binary_op(op, lhs, rhs),
    );

    // :numeric-fluents
    let multi_op = map(
        parens(tuple((
            parse_multi_op,
            preceded(multispace1, parse_f_exp),
            preceded(multispace1, space_separated_list1(parse_f_exp)),
        ))),
        |(op, lhs, rhs)| FExp::new_multi_op(op, lhs, rhs),
    );

    // :numeric-fluents
    let negated = map(
        parens(preceded(tuple((char('-'), multispace0)), parse_f_exp)),
        FExp::new_negative,
    );

    // :numeric-fluents
    let f_head = map(parse_f_head, FExp::new_f_head);

    alt((number, binary_op, multi_op, negated, f_head))(input)
}
