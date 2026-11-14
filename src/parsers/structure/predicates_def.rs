//! Provides parsers for predicate definitions (`<predicates-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_atomic_formula_skeleton, prefix};
use crate::types::PredicateDefinitions;

/// Parses a predicate definition (`<predicates-def>`), i.e. `(:predicates <atomic formula skeleton>⁺)`.
pub(crate) fn parse_predicates_def(node: &Node) -> R<PredicateDefinitions> {
    prefix(node, ":predicates", |args| {
        args.list1(parse_atomic_formula_skeleton)
            .map(PredicateDefinitions::new)
    })
}

crate::parsers::impl_parser!(PredicateDefinitions => parse_predicates_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = r"(:predicates
                             (at ?x - physob ?y - location)
                             (in ?x ?y - physob)
                        )";

        check!(
            PredicateDefinitions::from_str(input).is_value(PredicateDefinitions::new(vec![
                AtomicFormulaSkeleton::new(
                    Predicate::from("at"),
                    TypedList::from_iter([
                        Variable::from("x").to_typed("physob"),
                        Variable::from("y").to_typed("location"),
                    ])
                ),
                AtomicFormulaSkeleton::new(
                    Predicate::from("in"),
                    TypedList::from_iter([
                        Variable::from("x").to_typed("physob"),
                        Variable::from("y").to_typed("physob"),
                    ])
                )
            ]))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = r"(:predicates
                            (at ?x - physob ?y - location)
                            (in ?x ?y - physob)
                       )";

        check!(
            PredicateDefinitions::from_str(input).is_value(PredicateDefinitions::new(vec![
                AtomicFormulaSkeleton::new(
                    Predicate::from("at"),
                    TypedList::from_iter([
                        Variable::from("x").to_typed("physob"),
                        Variable::from("y").to_typed("location"),
                    ])
                ),
                AtomicFormulaSkeleton::new(
                    Predicate::from("in"),
                    TypedList::from_iter([
                        Variable::from("x").to_typed("physob"),
                        Variable::from("y").to_typed("physob"),
                    ])
                )
            ]))
        );
    }
}
