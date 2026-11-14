//! Provides parsers for duration operators.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::d_op::names;
use crate::types::DurationOperator;

/// Parses a duration operator, ignoring case.
pub(crate) fn parse_d_op(node: &Node) -> R<DurationOperator> {
    word(
        node,
        &[
            (names::LESS_THAN_OR_EQUAL, DurationOperator::LessThanOrEqual),
            (names::GREATER_OR_EQUAL, DurationOperator::GreaterOrEqual),
            (names::EQUAL, DurationOperator::Equal),
        ],
    )
}

crate::parsers::impl_parser!(DurationOperator => parse_d_op);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{DurationOperator, Parser};

    #[test]
    fn test_parse() {
        assert!(DurationOperator::from_str("<=").is_value(DurationOperator::LessThanOrEqual));
        assert!(DurationOperator::from_str("<").is_err());
        assert!(DurationOperator::from_str("(<=)").is_err());
    }
}
