//! Provides parsers for metric specification.

use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

use crate::parsers::{parse_metric_f_exp, parse_optimization, prefix_expr, ParseResult, Span};
use crate::types::MetricSpec;

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
pub fn parse_problem_metric_spec<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, MetricSpec> {
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

impl crate::parsers::Parser for MetricSpec {
    type Item = MetricSpec;

    /// See [`parse_problem_metric_spec`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_metric_spec(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{MetricFExp, MetricSpec, Optimization, Parser};

    #[test]
    fn test_parse() {
        assert!(
            MetricSpec::parse("(:metric minimize total-time)").is_value(MetricSpec::new(
                Optimization::Minimize,
                MetricFExp::TotalTime
            ))
        );
    }
}
