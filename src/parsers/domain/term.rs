//! Provides parsers for terms.

use crate::parsers::domain::{parse_function_term, parse_variable};
use crate::parsers::utility::parse_name;
use crate::types::domain::Term;
use nom::error::ErrorKind;
use nom::{error_position, IResult};

/// Parses a term, i.e. `<name> | <variable> | <function-term>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_term;
/// # use pddl::types::domain::Term;
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
        return Ok((remaining, Term::FunctionTerm(ft)));
    }

    return Err(nom::Err::Error(error_position!(input, ErrorKind::Alt)));
}
