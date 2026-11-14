//! Provides parsers for assignment operators.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::assign_op::names;
use crate::types::AssignOp;

/// Parses an assignment operator, ignoring case.
pub(crate) fn parse_assign_op(node: &Node) -> R<AssignOp> {
    word(
        node,
        &[
            (names::CHANGE, AssignOp::Assign),
            (names::ASSIGN, AssignOp::Assign),
            (names::SCALE_UP, AssignOp::ScaleUp),
            (names::SCALE_DOWN, AssignOp::ScaleDown),
            (names::INCREASE, AssignOp::Increase),
            (names::DECREASE, AssignOp::Decrease),
        ],
    )
}

crate::parsers::impl_parser!(AssignOp => parse_assign_op);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{AssignOp, Parser};

    #[test]
    fn test_parse() {
        assert!(AssignOp::from_str("scale-up").is_value(AssignOp::ScaleUp));
        assert!(AssignOp::from_str("scale").is_err());
        assert!(AssignOp::from_str("(scale-up)").is_err());
    }
}
