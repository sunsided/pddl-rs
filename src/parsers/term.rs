//! Provides parsers for terms.

use crate::parsers::{parse_function_term, parse_variable};
use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::Term;
use nom::error::ErrorKind;
use nom::error_position;

/// Parses a term, i.e. `<name> | <variable> | <function-term>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_term, preamble::*};
/// # use pddl::Term;
/// assert!(parse_term("abcde").is_value(Term::Name("abcde".into())));
/// assert!(parse_term("?abcde").is_value(Term::Variable("abcde".into())));
///```
pub fn parse_term<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Term> {
    let input = input.into();

    if let Ok((remaining, variable)) = parse_variable(input) {
        return Ok((remaining, Term::Variable(variable)));
    }

    if let Ok((remaining, name)) = parse_name(input) {
        return Ok((remaining, Term::Name(name)));
    }

    if let Ok((remaining, ft)) = parse_function_term(input) {
        return Ok((remaining, Term::Function(ft)));
    }

    return Err(nom::Err::Error(error_position!(
        input.into(),
        ErrorKind::Alt
    )));
}

impl crate::parsers::Parser for Term {
    type Item = Term;

    /// Parses a term.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Term, Parser};
    /// let (_, value) = Term::parse("some-name").unwrap();
    /// assert_eq!(value, Term::Name("some-name".into()));
    ///
    /// let (_, value) = Term::parse("?some-var").unwrap();
    /// assert_eq!(value, Term::Variable("some-var".into()));
    ///```
    ///
    /// ## See also
    /// See [`parse_term`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_term(input)
    }
}
