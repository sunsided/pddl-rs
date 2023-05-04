//! Provides parsers for assignment operations.

use crate::parsers::{ParseResult, Span};
use crate::types::time_specifier::names;
use crate::types::TimeSpecifier;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_time_specifier, preamble::*};
/// # use pddl::{TimeSpecifier};
/// assert!(parse_time_specifier("start".into()).is_value(TimeSpecifier::Start));
/// assert!(parse_time_specifier("end".into()).is_value(TimeSpecifier::End));
///```
pub fn parse_time_specifier(input: Span) -> ParseResult<TimeSpecifier> {
    map(alt((tag(names::START), tag(names::END))), |x: Span| {
        TimeSpecifier::try_from(*x.fragment()).expect("unhandled variant")
    })(input)
}

impl<'a> crate::parsers::Parser<'a> for TimeSpecifier {
    type Item = TimeSpecifier;

    /// See [`parse_time_specifier`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_time_specifier(input)
    }
}
