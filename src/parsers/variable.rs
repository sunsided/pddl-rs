//! Provides parsers for variables.

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::preceded;

use crate::parsers::{parse_name, ParseResult, Span};
use crate::types::Variable;

/// Parses a variable, i.e. `?<name>` and returns its name.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_variable, preamble::*};
/// assert!(parse_variable(Span::new("?abcde")).is_value("abcde".into()));
/// assert!(parse_variable(Span::new("?a-1_2")).is_value("a-1_2".into()));
/// assert!(parse_variable(Span::new("?Z01")).is_value("Z01".into()));
/// assert!(parse_variable(Span::new("?x-_-_")).is_value("x-_-_".into()));
///
/// assert!(parse_variable(Span::new("abcde")).is_err());
/// assert!(parse_variable(Span::new("?-")).is_err());
/// assert!(parse_variable(Span::new("?1")).is_err());
///```
pub fn parse_variable<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Variable> {
    map(preceded(tag("?"), parse_name), Variable::from)(input.into())
}

impl crate::parsers::Parser for Variable {
    type Item = Variable;

    /// Parses a variable.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Variable, Parser};
    /// let (_, value) = Variable::parse("?abcde").unwrap();
    /// assert_eq!(value, "abcde".into());
    ///```
    ///
    /// ## See also
    /// See [`parse_variable`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_variable(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Parser, Variable};

    #[test]
    fn test_parse() {
        let (_, value) = Variable::parse("?abcde").unwrap();
        assert_eq!(value, "abcde".into());
    }
}
