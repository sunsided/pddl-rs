//! Provides parsers for assignment operators (`<assign-op>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::expressions::assign_op::names;
use crate::types::AssignOp;

/// Parses an assignment operator (`<assign-op>`), ignoring case.
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
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(AssignOp::from_str("scale-up").is_value(AssignOp::ScaleUp));
        check!(AssignOp::from_str("scale").is_err());
        check!(AssignOp::from_str("(scale-up)").is_err());
    }
}
