//! Provides parsers for metric specifications.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_metric_f_exp, parse_optimization, prefix};
use crate::types::MetricSpec;

/// Parses a metric specification, i.e. `(:metric <optimization> <metric-f-exp>)`.
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

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            MetricSpec::from_str("(:metric minimize total-time)").is_value(MetricSpec::new(
                Optimization::Minimize,
                MetricFluentExpression::TotalTime
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            MetricSpec::from_str("(:metric minimize total-time)").is_value(MetricSpec::new(
                Optimization::Minimize,
                MetricFluentExpression::TotalTime
            ))
        );
    }
}
