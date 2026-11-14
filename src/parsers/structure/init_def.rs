//! Provides parsers for initial state definitions (`<init>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_init_el, prefix};
use crate::types::InitElements;

/// Parses an initial state definition (`<init>`), i.e. `(:init <init-el>*)`.
pub(crate) fn parse_problem_init_def(node: &Node) -> R<InitElements> {
    prefix(node, ":init", |args| {
        args.list0(parse_init_el).map(InitElements::new)
    })
}

crate::parsers::impl_parser!(InitElements => parse_problem_init_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:init (train-not-in-use train1) (at 10 (train-not-in-use train2)))";
        assert!(
            InitElements::from_str(input).is_value(InitElements::from_iter([
                InitElement::new_literal(NameLiteral::new(AtomicFormula::new_predicate(
                    "train-not-in-use".into(),
                    ["train1".into()]
                ))),
                InitElement::new_at(
                    Number::from(10),
                    NameLiteral::new(AtomicFormula::new_predicate(
                        "train-not-in-use".into(),
                        ["train2".into()]
                    ))
                )
            ]))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "(:init (train-not-in-use train1) (at 10 (train-not-in-use train2)))";
        assert!(
            InitElements::from_str(input).is_value(InitElements::from_iter([
                InitElement::new_literal(NameLiteral::new(AtomicFormula::new_predicate(
                    "train-not-in-use".into(),
                    ["train1".into()]
                ))),
                InitElement::new_at(
                    Number::from(10),
                    NameLiteral::new(AtomicFormula::new_predicate(
                        "train-not-in-use".into(),
                        ["train2".into()]
                    ))
                )
            ]))
        );
    }
}
