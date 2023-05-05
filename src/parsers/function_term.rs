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
/// assert!(parse_function_term("(fun-sym)").is_value(FunctionTerm::new("fun-sym".into(), vec![])));
///
/// let x = Term::Name("x".into());
/// assert!(parse_function_term("(fun-sym x)").is_value(FunctionTerm::new("fun-sym".into(), vec![x])));
///
/// let x = Term::Name("x".into());
/// let y = Term::Variable("y".into());
/// assert!(parse_function_term("(fun-sym ?y x)").is_value(FunctionTerm::new("fun-sym".into(), vec![y, x])));
///
/// let x = Term::Name("x".into());
/// let y = Term::Variable("y".into());
/// let a = Term::Name("a".into());
/// let ft = Term::Function(FunctionTerm::new(FunctionSymbol::from("fn"), vec![a]));
/// assert!(parse_function_term("(fun-sym ?y x (fn a))").is_value(FunctionTerm::new("fun-sym".into(), vec![y, x, ft])));
///```
pub fn parse_function_term<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FunctionTerm> {
    map(
        delimited(
            tag("("),
            tuple((parse_function_symbol, space_separated_list0(parse_term))),
            tag(")"),
        ),
        |(symbol, terms)| FunctionTerm::new(symbol, terms),
    )(input.into())
}

impl crate::parsers::Parser for FunctionTerm {
    type Item = FunctionTerm;

    /// See [`parse_function_term`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_function_term(input)
    }
}
