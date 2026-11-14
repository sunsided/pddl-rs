//! Provides parsers for binary operators (`<binary-op>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::binary_op::names;
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

    #[test]
    fn test_parse() {
        assert!(BinaryOp::from_str("/").is_value(BinaryOp::Division));
        assert!(BinaryOp::from_str("%").is_err());
        assert!(BinaryOp::from_str("(/)").is_err());
    }
}
