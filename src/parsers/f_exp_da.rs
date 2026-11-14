//! Provides parsers for durative action fluent expressions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, op_expr, parse_f_exp};
use crate::types::DurativeActionFluentExpression;

/// Parses a durative action fluent expression, i.e. `?duration | <f-exp>`, or an
/// arithmetic expression over them.
pub(crate) fn parse_f_exp_da(node: &Node) -> R<DurativeActionFluentExpression> {
    type E = DurativeActionFluentExpression;
    alt(
        node,
        &[
            &|n| {
                if n.is("?duration") {
                    Ok(E::new_duration())
                } else {
                    Err(n.error())
                }
            },
            &|n| {
                op_expr(
                    n,
                    parse_f_exp_da,
                    E::new_negative,
                    E::new_binary_op,
                    E::new_multi_op,
                )
            },
            &|n| parse_f_exp(n).map(E::new_f_exp),
        ],
    )
}

crate::parsers::impl_parser!(DurativeActionFluentExpression => parse_f_exp_da);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(DurativeActionFluentExpression::from_str("?duration")
            .is_value(DurativeActionFluentExpression::Duration));

        assert!(
            DurativeActionFluentExpression::from_str("(+ 1.23 2.34)").is_value(
                DurativeActionFluentExpression::new_binary_op(
                    BinaryOp::Addition,
                    DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(1.23)),
                    DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(2.34))
                )
            )
        );

        assert!(
            DurativeActionFluentExpression::from_str("(+ 1.23 2.34 3.45)").is_value(
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

    #[test]
    fn doc_example_0() {
        assert!(DurativeActionFluentExpression::from_str("?duration")
            .is_value(DurativeActionFluentExpression::Duration));

        assert!(
            DurativeActionFluentExpression::from_str("(+ 1.23 2.34)").is_value(
                DurativeActionFluentExpression::new_binary_op(
                    BinaryOp::Addition,
                    DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(1.23)),
                    DurativeActionFluentExpression::new_f_exp(FluentExpression::new_number(2.34))
                )
            )
        );

        assert!(
            DurativeActionFluentExpression::from_str("(+ 1.23 2.34 3.45)").is_value(
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
