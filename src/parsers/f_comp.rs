//! Provides parsers for f-comps.

use crate::parsers::{parens, ParseResult, Span};
use crate::parsers::{parse_binary_comp, parse_f_exp};
use crate::types::FComp;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses an f-comp.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_comp, preamble::*};
/// # use pddl::{FunctionTerm, Variable, FunctionSymbol, Term, FComp, BinaryComp, FExp, BinaryOp};
/// assert!(parse_f_comp("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
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
/// ));
///```
pub fn parse_f_comp<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FComp> {
    map(
        parens(tuple((
            parse_binary_comp,
            preceded(multispace1, parse_f_exp),
            preceded(multispace1, parse_f_exp),
        ))),
        FComp::from,
    )(input.into())
}

impl crate::parsers::Parser for FComp {
    type Item = FComp;

    /// See [`parse_f_comp`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_comp(input)
    }
}
