//! Provides parsers for binary operators (`<binary-op>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::expressions::binary_op::names;
use crate::types::BinaryOp;

/// Parses a binary operator (`<binary-op>`), ignoring case.
pub(crate) fn parse_binary_op(node: &Node) -> R<BinaryOp> {
    word(
        node,
        &[
            (names::MULTIPLICATION, BinaryOp::Multiplication),
            (names::ADDITION, BinaryOp::Addition),
            (names::SUBTRACTION, BinaryOp::Subtraction),
            (names::DIVISION, BinaryOp::Division),
        ],
    )
}

crate::parsers::impl_parser!(BinaryOp => parse_binary_op);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{BinaryOp, Parser};
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(BinaryOp::from_str("/").is_value(BinaryOp::Division));
        check!(BinaryOp::from_str("%").is_err());
        check!(BinaryOp::from_str("(/)").is_err());
    }
}
