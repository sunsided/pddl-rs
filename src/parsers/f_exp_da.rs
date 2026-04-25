//! Provides parsers for f-exp-da values..

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::{parens, space_separated_list1, ParseResult, Span};
use crate::parsers::{parse_binary_op, parse_f_exp, parse_multi_op};
use crate::types::DurativeActionFluentExpression;

/// Parses an f-exp-da.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_exp_da, preamble::*};
/// # use pddl::{BinaryOp, DurativeActionFluentExpression, FluentExpression, FunctionSymbol, MultiOp};
/// assert!(parse_f_exp_da("?duration").is_value(
///     DurativeActionFluentExpression::Duration
/// ));
///
/// assert!(parse_f_exp_da("(+ 1.23 2.34)").is_value(
///     DurativeActionFluentExpression::new_binary_op(
///         BinaryOp::Addition,
///             DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(1.23)),
///             DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(2.34))
///     )
/// ));
///
/// assert!(parse_f_exp_da("(+ 1.23 2.34 3.45)").is_value(
///     DurativeActionFluentExpression::new_multi_op(
///         MultiOp::Addition,
///         DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(1.23)),
///         [FluentExpression::new_number(2.34).into(), FluentExpression::new_number(3.45).into()]
///     )
/// ));
///```
pub fn parse_f_exp_da<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, DurativeActionFluentExpression> {
    // :duration-inequalities
    let duration = map(tag("?duration"), |_| {
        DurativeActionFluentExpression::new_duration()
    });

    let binary_op = map(
        parens((
            parse_binary_op,
            preceded(multispace1, parse_f_exp_da),
            preceded(multispace1, parse_f_exp_da),
        )),
        |(op, lhs, rhs)| DurativeActionFluentExpression::new_binary_op(op, lhs, rhs),
    );

    let multi_op = map(
        parens((
            parse_multi_op,
            preceded(multispace1, parse_f_exp_da),
            preceded(multispace1, space_separated_list1(parse_f_exp_da)),
        )),
        |(op, lhs, rhs)| DurativeActionFluentExpression::new_multi_op(op, lhs, rhs),
    );

    let negated = map(
        parens(preceded((char('-'), multispace0), parse_f_exp_da)),
        DurativeActionFluentExpression::new_negative,
    );

    let f_exp = map(parse_f_exp, DurativeActionFluentExpression::new_f_exp);

    alt((duration, binary_op, multi_op, negated, f_exp)).parse(input.into())
}

impl crate::parsers::Parser for DurativeActionFluentExpression {
    type Item = DurativeActionFluentExpression;

    /// See [`parse_f_exp_da`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_exp_da(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{BinaryOp, DurativeActionFluentExpression, FluentExpression, MultiOp, Parser};

    #[test]
    fn test_parse() {
        assert!(DurativeActionFluentExpression::parse("?duration")
            .is_value(DurativeActionFluentExpression::Duration));

        assert!(
            DurativeActionFluentExpression::parse("(+ 1.23 2.34)").is_value(
                DurativeActionFluentExpression::new_binary_op(
                    BinaryOp::Addition,
                    DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(1.23)),
                    DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(2.34))
                )
            )
        );

        assert!(
            DurativeActionFluentExpression::parse("(+ 1.23 2.34 3.45)").is_value(
                DurativeActionFluentExpression::new_multi_op(
                    MultiOp::Addition,
                    DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(1.23)),
                    [
                        FluentExpression::new_number(2.34).into(),
                        FluentExpression::new_number(3.45).into()
                    ]
                )
            )
        );
    }
}
