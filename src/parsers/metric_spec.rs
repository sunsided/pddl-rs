//! Provides parsers for metric specification.

use crate::parsers::{parse_metric_f_exp, parse_optimization, prefix_expr};
use crate::types::MetricSpec;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a metric specification.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_problem_metric_spec;
/// # use pddl::types::{MetricFExp, MetricSpec, Optimization};
/// assert_eq!(parse_problem_metric_spec("(:metric minimize total-time)"), Ok(("",
///     MetricSpec::new(
///         Optimization::Minimize,
///         MetricFExp::TotalTime
///     )
/// )));
///```
pub fn parse_problem_metric_spec(input: &str) -> IResult<&str, MetricSpec> {
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
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for MetricSpec<'a> {
    type Item = MetricSpec<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_problem_metric_spec(input)
    }
}
