//! Provides parsers for function types.

use crate::parsers::error::R;
use crate::parsers::parse_type;
use crate::parsers::sexpr::Node;
use crate::types::FunctionType;

/// Parses a function type, i.e. `<type>`.
pub(crate) fn parse_function_type(node: &Node) -> R<FunctionType> {
    parse_type(node).map(FunctionType::from)
}

crate::parsers::impl_parser!(FunctionType => parse_function_type);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{FunctionType, Parser, Type};

    #[test]
    fn test_parse() {
        assert!(FunctionType::from_str("number")
            .is_value(FunctionType::new(Type::Exactly("number".into()))));
        assert!(FunctionType::from_str("(either object number)")
            .is_value(FunctionType::new(Type::from_iter(["object", "number"]))));
    }
}
