//! Provides parsers for assignment operations.

use crate::parsers::{ParseResult, Span};
use crate::types::interval::names;
use crate::types::Interval;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses an intervals, i.e. `all`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_interval, preamble::*};
/// # use pddl::{Interval};
/// assert!(parse_interval("all").is_value(Interval::All));
///```
pub fn parse_interval<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Interval> {
    map(tag(names::ALL), |x: Span| {
        Interval::try_from(*x.fragment()).expect("unhandled variant")
    })(input.into())
}

impl crate::parsers::Parser for Interval {
    type Item = Interval;

    /// See [`parse_interval`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_interval(input)
    }
}
