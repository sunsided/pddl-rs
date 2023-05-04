//! Provides parsers for f-heads.

use crate::parsers::{parens, space_separated_list0, ParseResult, Span};
use crate::parsers::{parse_function_symbol, parse_term};
use crate::types::FHead;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses an f-head.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_head, preamble::*};
/// # use pddl::{FunctionTerm, Variable, FunctionSymbol, Term, FHead};
/// assert!(parse_f_head("fun-sym".into()).is_value(
///     FHead::new(FunctionSymbol::from_str("fun-sym"))
/// ));
///
/// assert!(parse_f_head("(fun-sym)".into()).is_value(
///     FHead::new(FunctionSymbol::from_str("fun-sym"))
/// ));
///
/// assert!(parse_f_head("(fun-sym term)".into()).is_value(
///     FHead::new_with_terms(FunctionSymbol::from_str("fun-sym"), [
///         Term::Name("term".into())
///     ])
/// ));
///```
pub fn parse_f_head(input: Span) -> ParseResult<FHead> {
    let simple = map(parse_function_symbol, FHead::new);
    let simple_parens = map(parens(parse_function_symbol), FHead::new);
    let with_terms = map(
        parens(tuple((
            parse_function_symbol,
            preceded(multispace1, space_separated_list0(parse_term)),
        ))),
        |(symbol, terms)| FHead::new_with_terms(symbol, terms),
    );

    alt((simple, simple_parens, with_terms))(input)
}

impl<'a> crate::parsers::Parser<'a> for FHead<'a> {
    type Item = FHead<'a>;

    /// See [`parse_f_head`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_f_head(input)
    }
}
