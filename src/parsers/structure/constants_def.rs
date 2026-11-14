//! Provides parsers for constant definitions (`<constants-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_name, prefix, typed_list};
use crate::types::Constants;

/// Parses a constant definition (`<constants-def>`), i.e. `(:constants <typed list (name)>)`.
pub(crate) fn parse_constants_def(node: &Node) -> R<Constants> {
    prefix(node, ":constants", |args| {
        typed_list(args, parse_name).map(Constants::new)
    })
}

crate::parsers::impl_parser!(Constants => parse_constants_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:constants B P D - physob)";
        check!(
            Constants::from_str(input).is_value(Constants::new(TypedList::from_iter([
                Name::from("B").to_typed("physob"),
                Name::from("P").to_typed("physob"),
                Name::from("D").to_typed("physob"),
            ])))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "(:constants B P D - physob)";
        check!(
            Constants::from_str(input).is_value(Constants::new(TypedList::from_iter([
                Name::from("B").to_typed("physob"),
                Name::from("P").to_typed("physob"),
                Name::from("D").to_typed("physob"),
            ])))
        );
    }
}
