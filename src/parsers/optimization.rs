//! Provides parsers for optimization.

use crate::parsers::{ParseResult, Span};
use crate::types::{optimization::names, Optimization};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::combinator::value;
use nom::Parser;

/// Parses an optimization goal, i.e. `minimize | maximize`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_optimization, preamble::*};
/// # use pddl::{Optimization};
/// assert!(parse_optimization("minimize").is_value(Optimization::Minimize));
/// assert!(parse_optimization("maximize").is_value(Optimization::Maximize));
///```
pub fn parse_optimization<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Optimization> {
    alt((
        value(Optimization::Minimize, tag_no_case(names::MINIMIZE)),
        value(Optimization::Maximize, tag_no_case(names::MAXIMIZE)),
    ))
    .parse(input.into())
}

impl crate::parsers::Parser for Optimization {
    type Item = Optimization;

    /// See [`parse_optimization`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_optimization(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::Optimization;

    #[test]
    fn test_parse() {
        assert!(Optimization::parse("minimize").is_value(Optimization::Minimize));
        assert!(Optimization::parse("maximize").is_value(Optimization::Maximize));
    }
}
