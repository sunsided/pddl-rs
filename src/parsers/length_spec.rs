//! Provides parsers for length specification.

use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::LengthSpec;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};

/// Parses a length specification. Deprecated since PDDL 2.1.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_length_spec, preamble::*};
/// # use pddl::LengthSpec;
/// assert!(parse_problem_length_spec("(:length)".into()).is_value(LengthSpec::default()));
/// assert!(parse_problem_length_spec("(:length (:serial 123))".into()).is_value(LengthSpec::new_serial(123)));
/// assert!(parse_problem_length_spec("(:length (:parallel 42))".into()).is_value(LengthSpec::new_parallel(42)));
/// assert!(parse_problem_length_spec("(:length (:serial 123) (:parallel 42))".into()).is_value(LengthSpec::new(Some(123), Some(42))));
///```
pub fn parse_problem_length_spec(input: Span) -> ParseResult<LengthSpec> {
    let serial = prefix_expr(":serial", nom::character::complete::u64);
    let parallel = prefix_expr(":parallel", nom::character::complete::u64);
    let length = prefix_expr(
        ":length",
        tuple((opt(serial), opt(preceded(multispace0, parallel)))),
    );

    map(length, |(serial, parallel)| {
        LengthSpec::new(serial, parallel)
    })(input)
}

impl<'a> crate::parsers::Parser<'a> for LengthSpec {
    type Item = LengthSpec;

    /// See [`parse_problem_length_spec`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_problem_length_spec(input)
    }
}
