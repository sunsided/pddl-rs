//! Provides parsers for length specification.

use crate::parsers::prefix_expr;
use crate::types::LengthSpec;
use nom::character::complete::{digit1, multispace0};
use nom::combinator::{map, map_res, opt, recognize};
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a length specification. Deprecated since PDDL 2.1.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_problem_length_spec;
/// # use pddl::types::LengthSpec;
/// assert_eq!(parse_problem_length_spec("(:length)"), Ok(("", LengthSpec::default())));
/// assert_eq!(parse_problem_length_spec("(:length (:serial 123))"), Ok(("", LengthSpec::new_serial(123))));
/// assert_eq!(parse_problem_length_spec("(:length (:parallel 42))"), Ok(("", LengthSpec::new_parallel(42))));
/// assert_eq!(parse_problem_length_spec("(:length (:serial 123) (:parallel 42))"), Ok(("", LengthSpec::new(Some(123), Some(42)))));
///```
pub fn parse_problem_length_spec(input: &str) -> IResult<&str, LengthSpec> {
    let serial = prefix_expr(":serial", map_res(recognize(digit1), str::parse));
    let parallel = prefix_expr(":parallel", map_res(recognize(digit1), str::parse));
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

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_problem_length_spec(input)
    }
}
