//! Provides parsers for basic function terms.

use crate::parsers::{
    parens, parse_function_symbol, parse_name, space_separated_list0, ws, ParseResult, Span,
};
use crate::types::BasicFunctionTerm;
use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::tuple;

/// Parses a basic function term.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_basic_function_term, UnwrapValue, Span};
/// # use pddl::{BasicFunctionTerm, Term};
/// assert!(parse_basic_function_term(Span::new("abcde")).is_value(
///     BasicFunctionTerm::new("abcde".into(), [])
/// ));
///
/// assert!(parse_basic_function_term(Span::new("(abcde)")).is_value(
///     BasicFunctionTerm::new("abcde".into(), [])
/// ));
///
/// assert!(parse_basic_function_term(Span::new("(abcde x y z)")).is_value(
///     BasicFunctionTerm::new("abcde".into(), ["x".into(), "y".into(), "z".into()])
/// ));
///```
pub fn parse_basic_function_term(input: Span) -> ParseResult<BasicFunctionTerm> {
    let direct = map(parse_function_symbol, |s| BasicFunctionTerm::new(s, []));
    let named = map(
        parens(tuple((
            parse_function_symbol,
            ws(space_separated_list0(parse_name)),
        ))),
        |(s, ns)| BasicFunctionTerm::new(s, ns),
    );
    alt((direct, named))(input)
}

impl<'a> crate::parsers::Parser<'a> for BasicFunctionTerm<'a> {
    type Item = BasicFunctionTerm<'a>;

    /// See [`parse_basic_function_term`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_basic_function_term(input)
    }
}
