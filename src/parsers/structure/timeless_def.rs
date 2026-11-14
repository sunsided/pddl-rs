//! Provides parsers for timeless definitions (`<timeless>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{literal, parse_name, prefix};
use crate::types::Timeless;

/// Parses a timeless definition (`<timeless>`), i.e. `(:timeless <literal (name)>⁺)`.
pub(crate) fn parse_timeless_def(node: &Node) -> R<Timeless> {
    prefix(node, ":timeless", |args| {
        args.list1(|n| literal(n, parse_name))
            .map(Timeless::from_iter)
    })
}

crate::parsers::impl_parser!(Timeless => parse_timeless_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:timeless (= x y) (= a b))";
        check!(Timeless::from_str(input).is_value(Timeless::from_iter([
            Literal::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                "x".into(),
                "y".into()
            ))),
            Literal::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                "a".into(),
                "b".into()
            )))
        ])));
    }

    #[test]
    fn doc_example_0() {
        let input = "(:timeless (= x y) (= a b))";
        check!(Timeless::from_str(input).is_value(Timeless::from_iter([
            Literal::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                "x".into(),
                "y".into()
            ))),
            Literal::AtomicFormula(
                // ...
                AtomicFormula::Equality(EqualityAtomicFormula::new("a".into(), "b".into()))
            )
        ])));
    }
}
