//! Provides parsers for assignment operations.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;

use crate::parsers::{ParseResult, Span};
use crate::types::time_specifier::names;
use crate::types::TimeSpecifier;

/// Parses an assignment operation, i.e. `increase | decrease`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_time_specifier, preamble::*};
/// # use pddl::{TimeSpecifier};
/// assert!(parse_time_specifier("start").is_value(TimeSpecifier::Start));
/// assert!(parse_time_specifier("end").is_value(TimeSpecifier::End));
///```
pub fn parse_time_specifier<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, TimeSpecifier> {
    map(alt((tag(names::START), tag(names::END))), |x: Span| {
        TimeSpecifier::try_from(*x.fragment()).expect("unhandled variant")
    })(input.into())
}

impl crate::parsers::Parser for TimeSpecifier {
    type Item = TimeSpecifier;

    /// See [`parse_time_specifier`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_time_specifier(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, TimeSpecifier};

    #[test]
    fn test_parse() {
        assert!(TimeSpecifier::parse("start").is_value(TimeSpecifier::Start));
        assert!(TimeSpecifier::parse("end").is_value(TimeSpecifier::End));
    }
}
