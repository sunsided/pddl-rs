//! Provides parsers for metric specification.

use crate::parsers::{parse_metric_f_exp, parse_optimization, prefix_expr, ParseResult, Span};
use crate::types::MetricSpec;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses a metric specification.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_metric_spec, preamble::*};
/// # use pddl::{MetricFExp, MetricSpec, Optimization};
/// assert!(parse_problem_metric_spec("(:metric minimize total-time)").is_value(
///     MetricSpec::new(
///         Optimization::Minimize,
///         MetricFExp::TotalTime
///     )
/// ));
///```
pub fn parse_problem_metric_spec<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, MetricSpec<'a>> {
    // :numeric-fluents
    map(
        prefix_expr(
            ":metric",
            tuple((
                parse_optimization,
                preceded(multispace1, parse_metric_f_exp),
            )),
        ),
        |(optimization, exp)| MetricSpec::new(optimization, exp),
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for MetricSpec<'a> {
    type Item = MetricSpec<'a>;

    /// See [`parse_problem_metric_spec`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_metric_spec(input.into())
    }
}
