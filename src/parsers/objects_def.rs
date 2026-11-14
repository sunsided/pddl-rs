//! Provides parsers for objects declarations.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_name, prefix, typed_list};
use crate::types::Objects;

/// Parses an objects declaration, i.e. `(:objects <typed list (name)>)`.
pub(crate) fn parse_problem_objects_declaration(node: &Node) -> R<Objects> {
    prefix(node, ":objects", |args| {
        typed_list(args, parse_name).map(Objects::new)
    })
}

crate::parsers::impl_parser!(Objects => parse_problem_objects_declaration);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:objects train1 train2)";
        assert!(Objects::from_str(input).is_value(Objects::new([
            Name::new("train1").to_typed(Type::OBJECT),
            Name::new("train2").to_typed(Type::OBJECT),
        ])));
    }

    #[test]
    fn doc_example_0() {
        let input = "(:objects train1 train2)";
        assert!(Objects::from_str(input).is_value(Objects::new([
            Name::new("train1").to_typed(Type::OBJECT),
            Name::new("train2").to_typed(Type::OBJECT),
        ])));
    }
}
