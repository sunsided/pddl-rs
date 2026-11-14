//! Provides parsers for fluent comparisons.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_binary_comp, parse_f_exp};
use crate::types::FluentComparison;

/// Parses a fluent comparison, i.e. `(<binary-comp> <f-exp> <f-exp>)`.
pub(crate) fn parse_f_comp(node: &Node) -> R<FluentComparison> {
    parens(node, |args| {
        let comparison = parse_binary_comp(args.next()?)?;
        let lhs = parse_f_exp(args.next()?)?;
        let rhs = parse_f_exp(args.next()?)?;
        Ok(FluentComparison::from((comparison, lhs, rhs)))
    })
}

crate::parsers::impl_parser!(FluentComparison => parse_f_comp);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            FluentComparison::from_str("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
                FluentComparison::new(
                    BinaryComparison::Equal,
                    FluentExpression::new_binary_op(
                        BinaryOp::Addition,
                        FluentExpression::new_number(1.23),
                        FluentExpression::new_number(2.34),
                    ),
                    FluentExpression::new_binary_op(
                        BinaryOp::Addition,
                        FluentExpression::new_number(1.23),
                        FluentExpression::new_number(2.34),
                    )
                )
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            FluentComparison::from_str("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
                FluentComparison::new(
                    BinaryComparison::Equal,
                    FluentExpression::new_binary_op(
                        BinaryOp::Addition,
                        FluentExpression::new_number(1.23),
                        FluentExpression::new_number(2.34),
                    ),
                    FluentExpression::new_binary_op(
                        BinaryOp::Addition,
                        FluentExpression::new_number(1.23),
                        FluentExpression::new_number(2.34),
                    )
                )
            )
        );
    }
}
