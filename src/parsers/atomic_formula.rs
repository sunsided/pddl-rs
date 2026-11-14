//! Provides the [`atomic_formula`] rule.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parens, parse_predicate};
use crate::types::AtomicFormula;

/// Parses an atomic formula, i.e. `(<predicate> t*) | (= t t)`, with `inner` for each `t`.
pub(crate) fn atomic_formula<'n, 'a, O>(
    node: &'n Node<'a>,
    inner: impl Fn(&'n Node<'a>) -> R<O>,
) -> R<AtomicFormula<O>> {
    if node.head() == Some("=") {
        return parens(node, |args| {
            args.next()?;
            let lhs = inner(args.next()?)?;
            let rhs = inner(args.next()?)?;
            Ok(AtomicFormula::Equality((lhs, rhs).into()))
        });
    }
    parens(node, |args| {
        let predicate = parse_predicate(args.next()?)?;
        let terms = args.list0(&inner)?;
        Ok(AtomicFormula::Predicate((predicate, terms).into()))
    })
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::parsers::{atomic_formula, parse_name, parse_one, parse_term};
    use crate::*;

    #[test]
    fn it_works() {
        let input = "(can-move ?from-waypoint ?to-waypoint)";
        let _effect = parse_one(input, |n| atomic_formula(n, parse_term)).unwrap();
    }

    #[test]
    fn test_parse() {
        assert!(
            parse_one("(= x y)", |n| atomic_formula(n, parse_name)).is_value(
                AtomicFormula::Equality(EqualityAtomicFormula::new("x".into(), "y".into()))
            )
        );
        assert!(
            parse_one("(move a b)", |n| atomic_formula(n, parse_name)).is_value(
                AtomicFormula::Predicate(PredicateAtomicFormula::new(
                    Predicate::from("move"),
                    vec!["a".into(), "b".into()]
                ))
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            parse_one("(= x y)", |n| atomic_formula(n, parse_name)).is_value(
                AtomicFormula::Equality(EqualityAtomicFormula::new("x".into(), "y".into()))
            )
        );
        assert!(
            parse_one("(move a b)", |n| atomic_formula(n, parse_name)).is_value(
                AtomicFormula::Predicate(PredicateAtomicFormula::new(
                    Predicate::from("move"),
                    vec!["a".into(), "b".into()]
                ))
            )
        );
    }
}
