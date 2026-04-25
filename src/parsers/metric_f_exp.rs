//! Provides parsers for metric f-exps.

use crate::parsers::{
    parens, parse_function_symbol, parse_name, parse_number, parse_pref_name, prefix_expr,
    space_separated_list0, space_separated_list1, ws, ParseResult, Span,
};
use crate::parsers::{parse_binary_op, parse_multi_op};
use crate::types::MetricFluentExpression;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

/// Parses a metric f-exp.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_metric_f_exp, preamble::*};
/// # use pddl::{BinaryOp, MetricFluentExpression, FunctionSymbol, MultiOp, Name, PreferenceName};
/// assert!(parse_metric_f_exp("1.23").is_value(
///     MetricFluentExpression::new_number(1.23)
/// ));
///
/// assert!(parse_metric_f_exp("(- total-time)").is_value(
///     MetricFluentExpression::new_negative(
///         MetricFluentExpression::TotalTime
///     )
/// ));
///
/// assert!(parse_metric_f_exp("(+ 1.23 2.34)").is_value(
///     MetricFluentExpression::new_binary_op(
///         BinaryOp::Addition,
///         MetricFluentExpression::new_number(1.23),
///         MetricFluentExpression::new_number(2.34)
///     )
/// ));
///
/// assert!(parse_metric_f_exp("(+ 1.23 2.34 3.45)").is_value(
///     MetricFluentExpression::new_multi_op(
///         MultiOp::Addition,
///         MetricFluentExpression::new_number(1.23),
///         [MetricFluentExpression::new_number(2.34), MetricFluentExpression::new_number(3.45)]
///     )
/// ));
///
/// assert!(parse_metric_f_exp("(is-violated preference)").is_value(
///     MetricFluentExpression::new_is_violated(
///         PreferenceName::from("preference")
///     )
/// ));
///
/// assert!(parse_metric_f_exp("fun-sym").is_value(
///     MetricFluentExpression::new_function(
///         FunctionSymbol::new_string("fun-sym"),
///         []
///     )
/// ));
///
/// assert!(parse_metric_f_exp("(fun-sym)").is_value(
///     MetricFluentExpression::new_function(
///         FunctionSymbol::new_string("fun-sym"),
///         []
///     )
/// ));
///
/// assert!(parse_metric_f_exp("(fun-sym a b c)").is_value(
///     MetricFluentExpression::new_function(
///         FunctionSymbol::new_string("fun-sym"),
///         [
///             Name::new("a"),
///             Name::new("b"),
///             Name::new("c")
///         ]
///     )
/// ));
///```
pub fn parse_metric_f_exp<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, MetricFluentExpression> {
    let binary_op = map(
        parens((
            parse_binary_op,
            preceded(multispace1, parse_metric_f_exp),
            preceded(multispace1, parse_metric_f_exp),
        )),
        |(op, lhs, rhs)| MetricFluentExpression::new_binary_op(op, lhs, rhs),
    );

    let multi_op = map(
        parens((
            parse_multi_op,
            preceded(multispace1, parse_metric_f_exp),
            preceded(multispace1, space_separated_list1(parse_metric_f_exp)),
        )),
        |(op, lhs, rhs)| MetricFluentExpression::new_multi_op(op, lhs, rhs),
    );

    let negated = map(
        parens(preceded((char('-'), multispace0), parse_metric_f_exp)),
        MetricFluentExpression::new_negative,
    );

    let number = map(parse_number, MetricFluentExpression::new_number);

    let simple_function = map(parse_function_symbol, |sym| {
        MetricFluentExpression::new_function(sym, [])
    });
    let complex_function = map(
        parens((parse_function_symbol, ws(space_separated_list0(parse_name)))),
        |(sym, names)| MetricFluentExpression::new_function(sym, names),
    );

    let total_time = map(tag("total-time"), |_| {
        MetricFluentExpression::new_total_time()
    });

    // :preferences
    let is_violated = map(
        prefix_expr("is-violated", parse_pref_name),
        MetricFluentExpression::new_is_violated,
    );

    alt((
        number,
        binary_op,
        multi_op,
        negated,
        total_time,
        is_violated,
        complex_function,
        simple_function,
    ))
    .parse(input.into())
}

impl crate::parsers::Parser for MetricFluentExpression {
    type Item = MetricFluentExpression;

    /// See [`parse_metric_f_exp`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_metric_f_exp(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{BinaryOp, FunctionSymbol, MetricFluentExpression, MultiOp, Name, PreferenceName};

    #[test]
    fn test_parse() {
        assert!(MetricFluentExpression::parse("1.23")
            .is_value(MetricFluentExpression::new_number(1.23)));

        assert!(MetricFluentExpression::parse("(- total-time)").is_value(
            MetricFluentExpression::new_negative(MetricFluentExpression::TotalTime)
        ));

        assert!(MetricFluentExpression::parse("(+ 1.23 2.34)").is_value(
            MetricFluentExpression::new_binary_op(
                BinaryOp::Addition,
                MetricFluentExpression::new_number(1.23),
                MetricFluentExpression::new_number(2.34)
            )
        ));

        assert!(
            MetricFluentExpression::parse("(+ 1.23 2.34 3.45)").is_value(
                MetricFluentExpression::new_multi_op(
                    MultiOp::Addition,
                    MetricFluentExpression::new_number(1.23),
                    [
                        MetricFluentExpression::new_number(2.34),
                        MetricFluentExpression::new_number(3.45)
                    ]
                )
            )
        );

        assert!(
            MetricFluentExpression::parse("(is-violated preference)").is_value(
                MetricFluentExpression::new_is_violated(PreferenceName::from("preference"))
            )
        );

        assert!(MetricFluentExpression::parse("fun-sym").is_value(
            MetricFluentExpression::new_function(FunctionSymbol::new_string("fun-sym"), [])
        ));

        assert!(MetricFluentExpression::parse("(fun-sym)").is_value(
            MetricFluentExpression::new_function(FunctionSymbol::new_string("fun-sym"), [])
        ));

        assert!(MetricFluentExpression::parse("(fun-sym a b c)").is_value(
            MetricFluentExpression::new_function(
                FunctionSymbol::new_string("fun-sym"),
                [Name::new("a"), Name::new("b"), Name::new("c")]
            )
        ));
    }
}
