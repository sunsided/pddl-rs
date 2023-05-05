//! Provides parsers for f-exps.

use crate::parsers::{parens, parse_number, space_separated_list1, ParseResult, Span};
use crate::parsers::{parse_binary_op, parse_f_head, parse_multi_op};
use crate::types::FExp;
use nom::branch::alt;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses an f-exp.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_exp, preamble::*};
/// # use pddl::{BinaryOp, FExp, FHead, FunctionSymbol, MultiOp};
/// assert!(parse_f_exp("1.23").is_value(
///     FExp::new_number(1.23)
/// ));
///
/// assert!(parse_f_exp("(+ 1.23 2.34)").is_value(
///     FExp::new_binary_op(
///         BinaryOp::Addition,
///         FExp::new_number(1.23),
///         FExp::new_number(2.34)
///     )
/// ));
///
/// assert!(parse_f_exp("(+ 1.23 2.34 3.45)").is_value(
///     FExp::new_multi_op(
///         MultiOp::Addition,
///         FExp::new_number(1.23),
///         [FExp::new_number(2.34), FExp::new_number(3.45)]
///     )
/// ));
///
/// assert!(parse_f_exp("(- 1.23)").is_value(
///     FExp::new_negative(FExp::new_number(1.23))
/// ));
///
/// assert!(parse_f_exp("fun-sym").is_value(
///     FExp::new_function(
///         FHead::new(FunctionSymbol::from_str("fun-sym"))
///     )
/// ));
///```
pub fn parse_f_exp<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FExp> {
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
    let f_head = map(parse_f_head, FExp::new_function);

    alt((number, binary_op, multi_op, negated, f_head))(input.into())
}

impl crate::parsers::Parser for FExp {
    type Item = FExp;

    /// See [`parse_f_exp`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_exp(input)
    }
}
