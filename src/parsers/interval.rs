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
/// assert!(parse_interval("all".into()).is_value(Interval::All));
///```
pub fn parse_interval(input: Span) -> ParseResult<Interval> {
    map(tag(names::ALL), |x: Span| {
        Interval::try_from(*x.fragment()).expect("unhandled variant")
    })(input)
}

impl<'a> crate::parsers::Parser<'a> for Interval {
    type Item = Interval;

    /// See [`parse_interval`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_interval(input.into())
    }
}
