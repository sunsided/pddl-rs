//! Provides parsers for f-exps.

use nom::branch::alt;
use nom::combinator::{map, map_opt};
use nom::Parser;

use crate::parsers::{parens, parse_number, space_separated_list1, NomError, ParseResult, Span};
use crate::parsers::{parse_binary_op, parse_f_head};
use crate::types::{BinaryOp, FluentExpression, MultiOp};

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
    let op = op_expr(
        parse_f_exp,
        FluentExpression::new_negative,
        FluentExpression::new_binary_op,
        FluentExpression::new_multi_op,
    );

    // :numeric-fluents
    let f_head = map(parse_f_head, FluentExpression::new_function);

    alt((number, op, f_head)).parse(input.into())
}

/// Parses `(<op> <arg>+)` as a negation, binary or multi op, chosen by the
/// argument count. Parsing the arguments once, instead of trying each form in
/// turn, keeps nested expressions such as `(- (- (- x)))` linear.
pub(crate) fn op_expr<'a, E>(
    arg: impl Parser<Span<'a>, Output = E, Error = NomError<'a>>,
    negative: fn(E) -> E,
    binary: fn(BinaryOp, E, E) -> E,
    multi: fn(MultiOp, E, Vec<E>) -> E,
) -> impl Parser<Span<'a>, Output = E, Error = NomError<'a>> {
    map_opt(
        parens((parse_binary_op, space_separated_list1(arg))),
        move |(op, mut args)| match args.len() {
            1 if op == BinaryOp::Subtraction => args.pop().map(negative),
            2 => {
                let rhs = args.pop()?;
                let lhs = args.pop()?;
                Some(binary(op, lhs, rhs))
            }
            3.. => {
                let op = MultiOp::try_from(op).ok()?;
                let rest = args.split_off(1);
                Some(multi(op, args.pop()?, rest))
            }
            _ => None,
        },
    )
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

        assert!(FluentExpression::parse("(- 1.23 2.34)").is_value(
            FluentExpression::new_binary_op(
                BinaryOp::Subtraction,
                FluentExpression::new_number(1.23),
                FluentExpression::new_number(2.34)
            )
        ));

        // Only `-` takes one argument, and only `+` and `*` take more than two.
        assert!(FluentExpression::from_str("(+ 1)").is_err());
        assert!(FluentExpression::from_str("(- 1 2 3)").is_err());
        assert!(FluentExpression::from_str("(/ 1 2 3)").is_err());

        assert!(
            FluentExpression::parse("fun-sym").is_value(FluentExpression::new_function(
                FunctionHead::new(FunctionSymbol::new_string("fun-sym"))
            ))
        );
    }

    #[test]
    fn nested_negation_parses_in_linear_time() {
        // Trying each operator form in turn parsed the argument again for every
        // form, about 3x work per level: depth 30 took hours.
        let input = format!("{}x{}", "(- ".repeat(30), ")".repeat(30));
        let start = std::time::Instant::now();
        assert!(FluentExpression::from_str(&input).is_ok());
        assert!(start.elapsed() < std::time::Duration::from_secs(1));
    }
}
