//! Provides parsers for terms.

use crate::parsers::parse_name;
use crate::parsers::{parse_function_term, parse_variable};
use crate::types::Term;
use nom::error::ErrorKind;
use nom::{error_position, IResult};

/// Parses a term, i.e. `<name> | <variable> | <function-term>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_term;
/// # use pddl::Term;
/// assert_eq!(parse_term("abcde"), Ok(("", Term::Name("abcde".into()))));
/// assert_eq!(parse_term("?abcde"), Ok(("", Term::Variable("abcde".into()))));
///```
pub fn parse_term(input: &str) -> IResult<&str, Term> {
    if let Ok((remaining, variable)) = parse_variable(input) {
        return Ok((remaining, Term::Variable(variable)));
    }

    if let Ok((remaining, name)) = parse_name(input) {
        return Ok((remaining, Term::Name(name)));
    }

    if let Ok((remaining, ft)) = parse_function_term(input) {
        return Ok((remaining, Term::Function(ft)));
    }

    return Err(nom::Err::Error(error_position!(input, ErrorKind::Alt)));
}

impl<'a> crate::parsers::Parser<'a> for Term<'a> {
    type Item = Term<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_term(input)
    }
}
