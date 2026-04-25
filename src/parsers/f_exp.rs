//! Provides parsers for f-exps.

use nom::branch::alt;
use nom::character::complete::{char, multispace0, multispace1};
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::{parens, parse_number, space_separated_list1, ParseResult, Span};
use crate::parsers::{parse_binary_op, parse_f_head, parse_multi_op};
use crate::types::FluentExpression;

/// Parses an f-exp.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_exp, preamble::*};
/// # use pddl::{BinaryOp, FluentExpression, FunctionHead, FunctionSymbol, MultiOp};
/// assert!(parse_f_exp("1.23").is_value(
///     FluentExpression::new_number(1.23)
/// ));
///
/// assert!(parse_f_exp("(+ 1.23 2.34)").is_value(
///     FluentExpression::new_binary_op(
///         BinaryOp::Addition,
///         FluentExpression::new_number(1.23),
///         FluentExpression::new_number(2.34)
///     )
/// ));
///
/// assert!(parse_f_exp("(+ 1.23 2.34 3.45)").is_value(
///     FluentExpression::new_multi_op(
///         MultiOp::Addition,
///         FluentExpression::new_number(1.23),
///         [FluentExpression::new_number(2.34), FluentExpression::new_number(3.45)]
///     )
/// ));
///
/// assert!(parse_f_exp("(- 1.23)").is_value(
///     FluentExpression::new_negative(FluentExpression::new_number(1.23))
/// ));
///
/// assert!(parse_f_exp("fun-sym").is_value(
///     FluentExpression::new_function(
///         FunctionHead::new(FunctionSymbol::new_string("fun-sym"))
///     )
/// ));
///```
pub fn parse_f_exp<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FluentExpression> {
    // :numeric-fluents
    let number = map(parse_number, FluentExpression::new_number);

    // :numeric-fluents
    let binary_op = map(
        parens((
            parse_binary_op,
            preceded(multispace1, parse_f_exp),
            preceded(multispace1, parse_f_exp),
        )),
        |(op, lhs, rhs)| FluentExpression::new_binary_op(op, lhs, rhs),
    );

    // :numeric-fluents
    let multi_op = map(
        parens((
            parse_multi_op,
            preceded(multispace1, parse_f_exp),
            preceded(multispace1, space_separated_list1(parse_f_exp)),
        )),
        |(op, lhs, rhs)| FluentExpression::new_multi_op(op, lhs, rhs),
    );

    // :numeric-fluents
    let negated = map(
        parens(preceded((char('-'), multispace0), parse_f_exp)),
        FluentExpression::new_negative,
    );

    // :numeric-fluents
    let f_head = map(parse_f_head, FluentExpression::new_function);

    alt((number, binary_op, multi_op, negated, f_head)).parse(input.into())
}

impl crate::parsers::Parser for FluentExpression {
    type Item = FluentExpression;

    /// See [`parse_f_exp`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_exp(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{BinaryOp, FluentExpression, FunctionHead, FunctionSymbol, MultiOp, Parser};

    #[test]
    fn test_parse() {
        assert!(FluentExpression::parse("1.23").is_value(FluentExpression::new_number(1.23)));

        assert!(FluentExpression::parse("(+ 1.23 2.34)").is_value(
            FluentExpression::new_binary_op(
                BinaryOp::Addition,
                FluentExpression::new_number(1.23),
                FluentExpression::new_number(2.34)
            )
        ));

        assert!(FluentExpression::parse("(+ 1.23 2.34 3.45)").is_value(
            FluentExpression::new_multi_op(
                MultiOp::Addition,
                FluentExpression::new_number(1.23),
                [
                    FluentExpression::new_number(2.34),
                    FluentExpression::new_number(3.45)
                ]
            )
        ));

        assert!(
            FluentExpression::parse("(- 1.23)").is_value(FluentExpression::new_negative(
                FluentExpression::new_number(1.23)
            ))
        );

        assert!(
            FluentExpression::parse("fun-sym").is_value(FluentExpression::new_function(
                FunctionHead::new(FunctionSymbol::new_string("fun-sym"))
            ))
        );
    }
}
