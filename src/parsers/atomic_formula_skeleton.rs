//! Provides parsers for atomic formula skeletons.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_predicate, parse_variable, typed_list};
use crate::types::AtomicFormulaSkeleton;

/// Parses an atomic formula skeleton, i.e. `(<predicate> <typed list (variable)>)`.
pub(crate) fn parse_atomic_formula_skeleton(node: &Node) -> R<AtomicFormulaSkeleton> {
    parens(node, |args| {
        let predicate = parse_predicate(args.next()?)?;
        let variables = typed_list(args, parse_variable)?;
        Ok(AtomicFormulaSkeleton::from((predicate, variables)))
    })
}

crate::parsers::impl_parser!(AtomicFormulaSkeleton => parse_atomic_formula_skeleton);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let value = AtomicFormulaSkeleton::from_str("(at ?x - physob ?y - location)").unwrap();

        assert_eq!(
            value,
            AtomicFormulaSkeleton::new(
                Predicate::from("at"),
                TypedList::from_iter([
                    Variable::from("x").to_typed("physob"),
                    Variable::from("y").to_typed("location")
                ])
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            AtomicFormulaSkeleton::from_str("(at ?x - physob ?y - location)").is_value(
                AtomicFormulaSkeleton::new(
                    Predicate::from("at"),
                    TypedList::from_iter([
                        Variable::from("x").to_typed("physob"),
                        Variable::from("y").to_typed("location")
                    ])
                )
            )
        );
    }

    #[test]
    fn doc_example_1() {
        let value = AtomicFormulaSkeleton::from_str("(at ?x - physob ?y - location)").unwrap();

        assert_eq!(
            value,
            AtomicFormulaSkeleton::new(
                Predicate::from("at"),
                TypedList::from_iter([
                    Variable::from("x").to_typed("physob"),
                    Variable::from("y").to_typed("location")
                ])
            )
        );
    }
}
