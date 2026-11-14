//! Provides parsers for type definitions (`<types-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_name, prefix, typed_list};
use crate::types::Types;

/// Parses a type definition (`<types-def>`), i.e. `(:types <typed list (name)>)`.
pub(crate) fn parse_types_def(node: &Node) -> R<Types> {
    prefix(node, ":types", |args| {
        typed_list(args, parse_name).map(Types::new)
    })
}

crate::parsers::impl_parser!(Types => parse_types_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:types location physob)";
        check!(
            Types::from_str(input).is_value(Types::new(TypedList::from_iter([
                Typed::new(Name::from("location"), Type::OBJECT),
                Typed::new(Name::from("physob"), Type::OBJECT),
            ])))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "(:types location physob)";
        check!(
            Types::from_str(input).is_value(Types::new(TypedList::from_iter([
                Typed::new(Name::from("location"), Type::OBJECT),
                Typed::new(Name::from("physob"), Type::OBJECT),
            ])))
        );
    }
}
