//! Provides parsers for fluent expressions (`<f-exp>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parens, parse_binary_op, parse_f_head, parse_number};
use crate::types::{BinaryOp, FluentExpression, MultiOp};

/// Parses a fluent expression (`<f-exp>`), i.e. `<number> | (<binary-op> <f-exp> <f-exp>)
/// | (<multi-op> <f-exp> <f-exp>⁺) | (- <f-exp>) | <f-head>`.
pub(crate) fn parse_f_exp(node: &Node) -> R<FluentExpression> {
    alt(
        node,
        &[
            &|n| parse_number(n).map(FluentExpression::new_number),
            &|n| {
                op_expr(
                    n,
                    parse_f_exp,
                    FluentExpression::new_negative,
                    FluentExpression::new_binary_op,
                    FluentExpression::new_multi_op,
                )
            },
            &|n| parse_f_head(n).map(FluentExpression::new_function),
        ],
    )
}

/// Parses an arithmetic expression `(<op> <arg>⁺)`: `-` with one argument is a
/// negation, any operator with two is a binary operation, and `*` or `+` with
/// more is a multi-operand operation.
pub(crate) fn op_expr<'n, 'a, E>(
    node: &'n Node<'a>,
    arg: impl Fn(&'n Node<'a>) -> R<E>,
    negative: fn(E) -> E,
    binary: fn(BinaryOp, E, E) -> E,
    multi: fn(MultiOp, E, Vec<E>) -> E,
) -> R<E> {
    parens(node, |args| {
        let op_node = args.next()?;
        let op = parse_binary_op(op_node)?;
        let mut operands = args.list1(&arg)?;
        match operands.len() {
            1 if op == BinaryOp::Subtraction => Ok(negative(operands.remove(0))),
            2 => {
                let rhs = operands.remove(1);
                let lhs = operands.remove(0);
                Ok(binary(op, lhs, rhs))
            }
            3.. => {
                let op = MultiOp::try_from(op).map_err(|_| op_node.error())?;
                let rest = operands.split_off(1);
                Ok(multi(op, operands.remove(0), rest))
            }
            _ => Err(op_node.error()),
        }
    })
}

crate::parsers::impl_parser!(FluentExpression => parse_f_exp);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        check!(FluentExpression::from_str("1.23").is_value(FluentExpression::new_number(1.23)));

        check!(FluentExpression::from_str("(+ 1.23 2.34)").is_value(
            FluentExpression::new_binary_op(
                BinaryOp::Addition,
                FluentExpression::new_number(1.23),
                FluentExpression::new_number(2.34)
            )
        ));

        check!(FluentExpression::from_str("(+ 1.23 2.34 3.45)").is_value(
            FluentExpression::new_multi_op(
                MultiOp::Addition,
                FluentExpression::new_number(1.23),
                [
                    FluentExpression::new_number(2.34),
                    FluentExpression::new_number(3.45)
                ]
            )
        ));

        check!(
            FluentExpression::from_str("(- 1.23)").is_value(FluentExpression::new_negative(
                FluentExpression::new_number(1.23)
            ))
        );

        check!(FluentExpression::from_str("(- 1.23 2.34)").is_value(
            FluentExpression::new_binary_op(
                BinaryOp::Subtraction,
                FluentExpression::new_number(1.23),
                FluentExpression::new_number(2.34)
            )
        ));

        // Only `-` takes one argument, and only `+` and `*` take more than two.
        check!(FluentExpression::from_str("(+ 1)").is_err());
        check!(FluentExpression::from_str("(- 1 2 3)").is_err());
        check!(FluentExpression::from_str("(/ 1 2 3)").is_err());

        check!(
            FluentExpression::from_str("fun-sym").is_value(FluentExpression::new_function(
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
        check!(FluentExpression::from_str(&input).is_ok());
        check!(start.elapsed() < std::time::Duration::from_secs(1));
    }

    #[test]
    fn doc_example_0() {
        check!(FluentExpression::from_str("1.23").is_value(FluentExpression::new_number(1.23)));

        check!(FluentExpression::from_str("(+ 1.23 2.34)").is_value(
            FluentExpression::new_binary_op(
                BinaryOp::Addition,
                FluentExpression::new_number(1.23),
                FluentExpression::new_number(2.34)
            )
        ));

        check!(FluentExpression::from_str("(+ 1.23 2.34 3.45)").is_value(
            FluentExpression::new_multi_op(
                MultiOp::Addition,
                FluentExpression::new_number(1.23),
                [
                    FluentExpression::new_number(2.34),
                    FluentExpression::new_number(3.45)
                ]
            )
        ));

        check!(
            FluentExpression::from_str("(- 1.23)").is_value(FluentExpression::new_negative(
                FluentExpression::new_number(1.23)
            ))
        );

        check!(
            FluentExpression::from_str("fun-sym").is_value(FluentExpression::new_function(
                FunctionHead::new(FunctionSymbol::new_string("fun-sym"))
            ))
        );
    }

    /// A space after an arithmetic operator is optional.
    #[test]
    fn operators_need_no_space_after_them() {
        let x = || FluentExpression::new_function(FunctionHead::new(FunctionSymbol::from("x")));
        check!(
            FluentExpression::from_str("(+x 1)")
                == Ok(FluentExpression::new_binary_op(
                    BinaryOp::Addition,
                    x(),
                    FluentExpression::new_number(1)
                ))
        );
        check!(FluentExpression::from_str("(-x)") == Ok(FluentExpression::new_negative(x())));
    }
}
