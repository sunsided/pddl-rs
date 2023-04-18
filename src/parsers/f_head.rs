//! Provides parsers for f-heads.

use crate::parsers::{parens, parse_function_symbol, parse_term, space_separated_list0};
use crate::types::FHead;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses an f-head.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_f_head;
/// # use pddl::types::{Name, FunctionTerm, Variable, FunctionSymbol, Term, FHead};
/// assert_eq!(parse_f_head("fun-sym"), Ok(("", FHead::new(FunctionSymbol::from_str("fun-sym")))));
/// assert_eq!(parse_f_head("(fun-sym term)"), Ok(("", FHead::new_with_terms(FunctionSymbol::from_str("fun-sym"), [Term::Name("term".into())]))));
///```
pub fn parse_f_head(input: &str) -> IResult<&str, FHead> {
    let simple = map(parse_function_symbol, FHead::new);
    let with_terms = map(
        parens(tuple((
            parse_function_symbol,
            preceded(multispace1, space_separated_list0(parse_term)),
        ))),
        |(symbol, terms)| FHead::new_with_terms(symbol, terms),
    );

    alt((simple, with_terms))(input)
}
