//! Provides parsers for types (`<type>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_primitive_type, prefix};
use crate::types::Type;

/// Parses a type (`<type>`), i.e. `<primitive-type> | (either <primitive-type>⁺)`.
pub(crate) fn parse_type(node: &Node) -> R<Type> {
    if node.atom().is_some() {
        return parse_primitive_type(node).map(Type::Exactly);
    }
    prefix(node, "either", |args| args.list1(parse_primitive_type)).map(Type::EitherOf)
}

crate::parsers::impl_parser!(Type => parse_type);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Parser, PrimitiveType, Type};

    #[test]
    fn explicit_works() {
        assert!(Type::from_str("object").is_value(Type::Exactly("object".into())));
    }

    #[test]
    fn either_works() {
        assert!(
            Type::from_str("(either object number)").is_value(Type::EitherOf(vec![
                PrimitiveType::from("object"),
                PrimitiveType::from("number")
            ]))
        );
        assert!(Type::from_str("(EITHER object number)")
            .is_value(Type::from_iter(["object", "number"])));
    }

    #[test]
    fn test_invalid() {
        assert!(Type::from_str("()").is_err());
        assert!(Type::from_str("(either)").is_err());
        assert!(Type::from_str("(object)").is_err());
    }
}
