//! Provides parsers for primitive types.

use crate::parsers::error::R;
use crate::parsers::parse_name;
use crate::parsers::sexpr::Node;
use crate::types::PrimitiveType;

/// Parses a primitive type, i.e. `object | <name>`.
pub(crate) fn parse_primitive_type(node: &Node) -> R<PrimitiveType> {
    parse_name(node).map(PrimitiveType::from)
}

crate::parsers::impl_parser!(PrimitiveType => parse_primitive_type);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, PrimitiveType};

    #[test]
    fn test_parse() {
        assert!(PrimitiveType::from_str("object").is_value("object".into()));
        assert!(PrimitiveType::from_str("number").is_value("number".into()));
        assert!(PrimitiveType::from_str("a-1_2").is_value("a-1_2".into()));
        assert!(PrimitiveType::from_str("obj!ect").is_err());
    }
}
