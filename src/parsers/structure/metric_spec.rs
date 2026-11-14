//! Provides parsers for metric specifications (`<metric-spec>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_metric_f_exp, parse_optimization, prefix};
use crate::types::MetricSpec;

/// Parses a metric specification (`<metric-spec>`), i.e. `(:metric <optimization> <metric-f-exp>)`.
pub(crate) fn parse_problem_metric_spec(node: &Node) -> R<MetricSpec> {
    prefix(node, ":metric", |args| {
        Ok(MetricSpec::new(
            parse_optimization(args.next()?)?,
            parse_metric_f_exp(args.next()?)?,
        ))
    })
}

crate::parsers::impl_parser!(MetricSpec => parse_problem_metric_spec);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        check!(
            MetricSpec::from_str("(:metric minimize total-time)").is_value(MetricSpec::new(
                Optimization::Minimize,
                MetricFluentExpression::TotalTime
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        check!(
            MetricSpec::from_str("(:metric minimize total-time)").is_value(MetricSpec::new(
                Optimization::Minimize,
                MetricFluentExpression::TotalTime
            ))
        );
    }
}
