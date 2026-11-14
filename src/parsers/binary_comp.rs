//! Provides parsers for binary comparisons.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::word;
use crate::types::binary_comp::names;
use crate::types::BinaryComparison;

/// Parses a binary comparison, ignoring case.
pub(crate) fn parse_binary_comp(node: &Node) -> R<BinaryComparison> {
    word(
        node,
        &[
            (
                names::GREATER_THAN_OR_EQUAL,
                BinaryComparison::GreaterOrEqual,
            ),
            (names::LESS_THAN_OR_EQUAL, BinaryComparison::LessThanOrEqual),
            (names::EQUAL, BinaryComparison::Equal),
            (names::GREATER_THAN, BinaryComparison::GreaterThan),
            (names::LESS_THAN, BinaryComparison::LessThan),
        ],
    )
}

crate::parsers::impl_parser!(BinaryComparison => parse_binary_comp);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{BinaryComparison, Parser};

    #[test]
    fn test_parse() {
        assert!(BinaryComparison::from_str(">=").is_value(BinaryComparison::GreaterOrEqual));
        assert!(BinaryComparison::from_str("=>").is_err());
        assert!(BinaryComparison::from_str("(>=)").is_err());
    }
}
