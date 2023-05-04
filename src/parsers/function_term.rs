//! Provides parsers for function terms.

use crate::parsers::{parse_function_symbol, parse_term};
use crate::parsers::{space_separated_list0, ParseResult, Span};
use crate::types::FunctionTerm;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::{delimited, tuple};

/// Parses a function terms, i.e. `(<function symbol> <term>*)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_function_term, preamble::*};
/// # use pddl::{FunctionTerm, Variable, FunctionSymbol, Term};
/// assert!(parse_function_term("(fun-sym)".into()).is_value(FunctionTerm::new("fun-sym".into(), vec![])));
///
/// let x = Term::Name("x".into());
/// assert!(parse_function_term("(fun-sym x)".into()).is_value(FunctionTerm::new("fun-sym".into(), vec![x])));
///
/// let x = Term::Name("x".into());
/// let y = Term::Variable("y".into());
/// assert!(parse_function_term("(fun-sym ?y x)".into()).is_value(FunctionTerm::new("fun-sym".into(), vec![y, x])));
///
/// let x = Term::Name("x".into());
/// let y = Term::Variable("y".into());
/// let a = Term::Name("a".into());
/// let ft = Term::Function(FunctionTerm::new(FunctionSymbol::from("fn"), vec![a]));
/// assert!(parse_function_term("(fun-sym ?y x (fn a))".into()).is_value(FunctionTerm::new("fun-sym".into(), vec![y, x, ft])));
///```
pub fn parse_function_term(input: Span) -> ParseResult<FunctionTerm> {
    map(
        delimited(
            tag("("),
            tuple((parse_function_symbol, space_separated_list0(parse_term))),
            tag(")"),
        ),
        |(symbol, terms)| FunctionTerm::new(symbol, terms),
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for FunctionTerm<'a> {
    type Item = FunctionTerm<'a>;

    /// See [`parse_function_term`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_function_term(input)
    }
}
