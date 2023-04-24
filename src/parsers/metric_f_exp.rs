//! Provides parsers for metric f-exps.

use crate::parsers::{
    parens, parse_function_symbol, parse_name, parse_number, parse_pref_name, prefix_expr,
    space_separated_list0, space_separated_list1, ws,
};
use crate::parsers::{parse_binary_op, parse_multi_op};
use crate::types::MetricFExp;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a metric f-exp.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_metric_f_exp;
/// # use pddl::{BinaryOp, MetricFExp, FunctionSymbol, MultiOp, Name, PreferenceName};
/// assert_eq!(parse_metric_f_exp("1.23"), Ok(("",
///     MetricFExp::new_number(1.23)
/// )));
///
/// assert_eq!(parse_metric_f_exp("(- total-time)"), Ok(("",
///     MetricFExp::new_negative(
///         MetricFExp::TotalTime
///     )
/// )));
///
/// assert_eq!(parse_metric_f_exp("(+ 1.23 2.34)"), Ok(("",
///     MetricFExp::new_binary_op(
///         BinaryOp::Addition,
///         MetricFExp::new_number(1.23),
///         MetricFExp::new_number(2.34)
///     )
/// )));
///
/// assert_eq!(parse_metric_f_exp("(+ 1.23 2.34 3.45)"), Ok(("",
///     MetricFExp::new_multi_op(
///         MultiOp::Addition,
///         MetricFExp::new_number(1.23),
///         [MetricFExp::new_number(2.34), MetricFExp::new_number(3.45)]
///     )
/// )));
///
/// assert_eq!(parse_metric_f_exp("(is-violated preference)"), Ok(("",
///     MetricFExp::new_is_violated(
///         PreferenceName::from("preference")
///     )
/// )));
///
/// assert_eq!(parse_metric_f_exp("fun-sym"), Ok(("",
///     MetricFExp::new_function(
///         FunctionSymbol::from_str("fun-sym"),
///         []
///     )
/// )));
///
/// assert_eq!(parse_metric_f_exp("(fun-sym)"), Ok(("",
///     MetricFExp::new_function(
///         FunctionSymbol::from_str("fun-sym"),
///         []
///     )
/// )));
///
/// assert_eq!(parse_metric_f_exp("(fun-sym a b c)"), Ok(("",
///     MetricFExp::new_function(
///         FunctionSymbol::from_str("fun-sym"),
///         [
///             Name::new("a"),
///             Name::new("b"),
///             Name::new("c")
///         ]
///     )
/// )));
///```
pub fn parse_metric_f_exp(input: &str) -> IResult<&str, MetricFExp> {
    let binary_op = map(
        parens(tuple((
            parse_binary_op,
            preceded(multispace1, parse_metric_f_exp),
            preceded(multispace1, parse_metric_f_exp),
        ))),
        |(op, lhs, rhs)| MetricFExp::new_binary_op(op, lhs, rhs),
    );

    let multi_op = map(
        parens(tuple((
            parse_multi_op,
            preceded(multispace1, parse_metric_f_exp),
            preceded(multispace1, space_separated_list1(parse_metric_f_exp)),
        ))),
        |(op, lhs, rhs)| MetricFExp::new_multi_op(op, lhs, rhs),
    );

    let negated = map(
        parens(preceded(
            tuple((char('-'), multispace0)),
            parse_metric_f_exp,
        )),
        MetricFExp::new_negative,
    );

    let number = map(parse_number, MetricFExp::new_number);

    let simple_function = map(parse_function_symbol, |sym| {
        MetricFExp::new_function(sym, [])
    });
    let complex_function = map(
        parens(tuple((
            parse_function_symbol,
            ws(space_separated_list0(parse_name)),
        ))),
        |(sym, names)| MetricFExp::new_function(sym, names),
    );

    let total_time = map(tag("total-time"), |_| MetricFExp::new_total_time());

    // :preferences
    let is_violated = map(
        prefix_expr("is-violated", parse_pref_name),
        MetricFExp::new_is_violated,
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
    ))(input)
}

impl<'a> crate::parsers::Parser<'a> for MetricFExp<'a> {
    type Item = MetricFExp<'a>;

    /// See [`parse_metric_f_exp`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_metric_f_exp(input)
    }
}
