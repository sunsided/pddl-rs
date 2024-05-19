//! Provides parsers for length specification.

use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};

use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::LengthSpec;

/// Parses a length specification. Deprecated since PDDL 2.1.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_length_spec, preamble::*};
/// # use pddl::LengthSpec;
/// assert!(parse_problem_length_spec("(:length)").is_value(LengthSpec::default()));
/// assert!(parse_problem_length_spec("(:length (:serial 123))").is_value(LengthSpec::new_serial(123)));
/// assert!(parse_problem_length_spec("(:length (:parallel 42))").is_value(LengthSpec::new_parallel(42)));
/// assert!(parse_problem_length_spec("(:length (:serial 123) (:parallel 42))").is_value(LengthSpec::new(Some(123), Some(42))));
///```
pub fn parse_problem_length_spec<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, LengthSpec> {
    let serial = prefix_expr(":serial", nom::character::complete::u64);
    let parallel = prefix_expr(":parallel", nom::character::complete::u64);
    let length = prefix_expr(
        ":length",
        tuple((opt(serial), opt(preceded(multispace0, parallel)))),
    );

    map(length, |(serial, parallel)| {
        LengthSpec::new(serial, parallel)
    })(input.into())
}

impl crate::parsers::Parser for LengthSpec {
    type Item = LengthSpec;

    /// See [`parse_problem_length_spec`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_length_spec(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{LengthSpec, Parser};

    #[test]
    fn test_parse() {
        assert!(LengthSpec::parse("(:length)").is_value(LengthSpec::default()));
        assert!(LengthSpec::parse("(:length (:serial 123))").is_value(LengthSpec::new_serial(123)));
        assert!(
            LengthSpec::parse("(:length (:parallel 42))").is_value(LengthSpec::new_parallel(42))
        );
        assert!(LengthSpec::parse("(:length (:serial 123) (:parallel 42))")
            .is_value(LengthSpec::new(Some(123), Some(42))));
    }
}
