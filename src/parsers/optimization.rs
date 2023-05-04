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
/// assert!(parse_optimization("minimize".into()).is_value(Optimization::Minimize));
/// assert!(parse_optimization("maximize".into()).is_value(Optimization::Maximize));
///```
pub fn parse_optimization(input: Span) -> ParseResult<Optimization> {
    map(
        alt((tag(names::MINIMIZE), tag(names::MAXIMIZE))),
        |x: Span| Optimization::try_from(*x.fragment()).expect("unhandled variant"),
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for Optimization {
    type Item = Optimization;

    /// See [`parse_optimization`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_optimization(input.into())
    }
}
