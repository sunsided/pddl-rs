//! Provides parsers for timed assignment operators (`<assign-op-t>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::assign_op_t::names;
use crate::types::TimedAssignOperator;

/// Parses a timed assignment operator (`<assign-op-t>`), ignoring case.
pub(crate) fn parse_assign_op_t(node: &Node) -> R<TimedAssignOperator> {
    word(
        node,
        &[
            (names::INCREASE, TimedAssignOperator::Increase),
            (names::DECREASE, TimedAssignOperator::Decrease),
        ],
    )
}

crate::parsers::impl_parser!(TimedAssignOperator => parse_assign_op_t);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, TimedAssignOperator};

    #[test]
    fn test_parse() {
        assert!(TimedAssignOperator::from_str("increase").is_value(TimedAssignOperator::Increase));
        assert!(TimedAssignOperator::from_str("assign").is_err());
        assert!(TimedAssignOperator::from_str("(increase)").is_err());
    }
}
