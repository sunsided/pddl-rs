//! Provides parsers for metric fluent expressions (`<metric-f-exp>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    alt, op_expr, parens, parse_function_symbol, parse_name, parse_number, parse_pref_name, prefix,
};
use crate::types::MetricFluentExpression;

/// Parses a metric fluent expression (`<metric-f-exp>`), i.e. `<number> | total-time | (is-violated <pref-name>)
/// | (<function-symbol> <name>*) | <function-symbol>`, or an arithmetic expression over them.
pub(crate) fn parse_metric_f_exp(node: &Node) -> R<MetricFluentExpression> {
    type M = MetricFluentExpression;
    alt(
        node,
        &[
            &|n| parse_number(n).map(M::new_number),
            &|n| {
                op_expr(
                    n,
                    parse_metric_f_exp,
                    M::new_negative,
                    M::new_binary_op,
                    M::new_multi_op,
                )
            },
            &|n| {
                if n.is("total-time") {
                    Ok(M::total_time())
                } else {
                    Err(n.error())
                }
            },
            // :preferences
            &|n| {
                prefix(n, "is-violated", |args| parse_pref_name(args.next()?))
                    .map(M::new_is_violated)
            },
            &|n| {
                parens(n, |args| {
                    Ok((
                        parse_function_symbol(args.next()?)?,
                        args.list0(parse_name)?,
                    ))
                })
                .map(|(symbol, names)| M::function(symbol, names))
            },
            &|n| parse_function_symbol(n).map(|symbol| M::function(symbol, [])),
        ],
    )
}

crate::parsers::impl_parser!(MetricFluentExpression => parse_metric_f_exp);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        check!(
            MetricFluentExpression::from_str("1.23").is_value(MetricFluentExpression::number(1.23))
        );

        check!(MetricFluentExpression::from_str("(- total-time)").is_value(
            MetricFluentExpression::negative(MetricFluentExpression::TotalTime)
        ));

        check!(MetricFluentExpression::from_str("(+ 1.23 2.34)").is_value(
            MetricFluentExpression::binary_op(
                BinaryOp::Addition,
                MetricFluentExpression::number(1.23),
                MetricFluentExpression::number(2.34)
            )
        ));

        check!(
            MetricFluentExpression::from_str("(+ 1.23 2.34 3.45)").is_value(
                MetricFluentExpression::multi_op(
                    MultiOp::Addition,
                    MetricFluentExpression::number(1.23),
                    [
                        MetricFluentExpression::number(2.34),
                        MetricFluentExpression::number(3.45)
                    ]
                )
            )
        );

        check!(
            MetricFluentExpression::from_str("(is-violated preference)").is_value(
                MetricFluentExpression::is_violated(PreferenceName::from("preference"))
            )
        );

        check!(MetricFluentExpression::from_str("fun-sym").is_value(
            MetricFluentExpression::function(FunctionSymbol::string("fun-sym"), [])
        ));

        check!(MetricFluentExpression::from_str("(fun-sym)").is_value(
            MetricFluentExpression::function(FunctionSymbol::string("fun-sym"), [])
        ));

        check!(
            MetricFluentExpression::from_str("(fun-sym a b c)").is_value(
                MetricFluentExpression::function(
                    FunctionSymbol::string("fun-sym"),
                    [Name::new("a"), Name::new("b"), Name::new("c")]
                )
            )
        );
    }

    #[test]
    fn doc_example_0() {
        check!(
            MetricFluentExpression::from_str("1.23").is_value(MetricFluentExpression::number(1.23))
        );

        check!(MetricFluentExpression::from_str("(- total-time)").is_value(
            MetricFluentExpression::negative(MetricFluentExpression::TotalTime)
        ));

        check!(MetricFluentExpression::from_str("(+ 1.23 2.34)").is_value(
            MetricFluentExpression::binary_op(
                BinaryOp::Addition,
                MetricFluentExpression::number(1.23),
                MetricFluentExpression::number(2.34)
            )
        ));

        check!(
            MetricFluentExpression::from_str("(+ 1.23 2.34 3.45)").is_value(
                MetricFluentExpression::multi_op(
                    MultiOp::Addition,
                    MetricFluentExpression::number(1.23),
                    [
                        MetricFluentExpression::number(2.34),
                        MetricFluentExpression::number(3.45)
                    ]
                )
            )
        );

        check!(
            MetricFluentExpression::from_str("(is-violated preference)").is_value(
                MetricFluentExpression::is_violated(PreferenceName::from("preference"))
            )
        );

        check!(MetricFluentExpression::from_str("fun-sym").is_value(
            MetricFluentExpression::function(FunctionSymbol::string("fun-sym"), [])
        ));

        check!(MetricFluentExpression::from_str("(fun-sym)").is_value(
            MetricFluentExpression::function(FunctionSymbol::string("fun-sym"), [])
        ));

        check!(
            MetricFluentExpression::from_str("(fun-sym a b c)").is_value(
                MetricFluentExpression::function(
                    FunctionSymbol::string("fun-sym"),
                    [Name::new("a"), Name::new("b"), Name::new("c")]
                )
            )
        );
    }
}
