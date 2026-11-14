//! Provides parsers for derived predicates.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_atomic_formula_skeleton, parse_gd, prefix};
use crate::types::DerivedPredicate;

/// Parses a derived predicate, i.e. `(:derived <atomic formula skeleton> <GD>)`.
pub(crate) fn parse_derived_predicate(node: &Node) -> R<DerivedPredicate> {
    prefix(node, ":derived", |args| {
        Ok(DerivedPredicate::from((
            parse_atomic_formula_skeleton(args.next()?)?,
            parse_gd(args.next()?)?,
        )))
    })
}

crate::parsers::impl_parser!(DerivedPredicate => parse_derived_predicate);

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn test_parse() {
        let input = r#"(:derived (train-usable ?t - train)
                            (and
                                (train-has-guard ?t)
                                (train-has-driver ?t)
                            )
                        )"#;

        let predicate = DerivedPredicate::from_str(input).unwrap();
        assert_eq!(predicate.predicate().name(), "train-usable");
        assert!(matches!(predicate.expression(), &GoalDefinition::And(..)));
    }

    #[test]
    fn doc_example_0() {
        let input = r#"(:derived (train-usable ?t - train)
                            (and
                                (train-has-guard ?t)
                                (train-has-driver ?t)
                            )
                        )"#;

        let predicate = DerivedPredicate::from_str(input).unwrap();
        assert_eq!(predicate.predicate().name(), "train-usable");
        assert!(matches!(predicate.expression(), &GoalDefinition::And(..)));
    }
}
