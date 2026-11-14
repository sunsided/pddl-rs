//! Provides parsers for multi-operand operators (`<multi-op>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::expressions::multi_op::names;
use crate::types::MultiOp;

/// Parses a multi-operand operator (`<multi-op>`), ignoring case.
pub(crate) fn parse_multi_op(node: &Node) -> R<MultiOp> {
    word(
        node,
        &[
            (names::MULTIPLICATION, MultiOp::Multiplication),
            (names::ADDITION, MultiOp::Addition),
        ],
    )
}

crate::parsers::impl_parser!(MultiOp => parse_multi_op);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{MultiOp, Parser};

    #[test]
    fn test_parse() {
        assert!(MultiOp::from_str("+").is_value(MultiOp::Addition));
        assert!(MultiOp::from_str("-").is_err());
        assert!(MultiOp::from_str("(+)").is_err());
    }
}
