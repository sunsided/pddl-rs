//! Provides parsers for function terms.

use crate::parsers::space_separated_list0;
use crate::parsers::{parse_function_symbol, parse_term};
use crate::types::FunctionTerm;
use nom::bytes::complete::tag;
use nom::sequence::{delimited, tuple};
use nom::IResult;

/// Parses a function terms, i.e. `(<function symbol> <term>*)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_function_term;
/// # use pddl::types::{FunctionTerm, Variable, FunctionSymbol, Term};
/// assert_eq!(parse_function_term("(fun-sym)"), Ok(("", FunctionTerm::new("fun-sym".into(), vec![]))));
///
/// let x = Term::Name("x".into());
/// assert_eq!(parse_function_term("(fun-sym x)"), Ok(("", FunctionTerm::new("fun-sym".into(), vec![x]))));
///
/// let x = Term::Name("x".into());
/// let y = Term::Variable("y".into());
/// assert_eq!(parse_function_term("(fun-sym ?y x)"), Ok(("", FunctionTerm::new("fun-sym".into(), vec![y, x]))));
///
/// let x = Term::Name("x".into());
/// let y = Term::Variable("y".into());
/// let a = Term::Name("a".into());
/// let ft = Term::Function(FunctionTerm::new(FunctionSymbol::from("fn"), vec![a]));
/// assert_eq!(parse_function_term("(fun-sym ?y x (fn a))"), Ok(("", FunctionTerm::new("fun-sym".into(), vec![y, x, ft]))));
///```
pub fn parse_function_term(input: &str) -> IResult<&str, FunctionTerm> {
    let (remaining, (symbol, terms)) = delimited(
        tag("("),
        tuple((parse_function_symbol, space_separated_list0(parse_term))),
        tag(")"),
    )(input)?;
    Ok((remaining, FunctionTerm::new(symbol, terms)))
}
