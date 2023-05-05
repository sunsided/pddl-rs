//! Provides parsers for optimization.

use crate::parsers::{ParseResult, Span};
use crate::types::{optimization::names, Optimization};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

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
    map(
        alt((tag(names::MINIMIZE), tag(names::MAXIMIZE))),
        |x: Span| Optimization::try_from(*x.fragment()).expect("unhandled variant"),
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for Optimization {
    type Item = Optimization;

    /// See [`parse_optimization`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_optimization(input)
    }
}
