//! Provides parsers for initial state elements.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, literal, parse_basic_function_term, parse_name, parse_number, prefix};
use crate::types::InitElement;

/// Parses an initial state element, i.e. `<literal (name)> | (at <number> <literal (name)>)
/// | (= <basic-function-term> <number>) | (= <basic-function-term> <name>)`.
pub(crate) fn parse_init_el(node: &Node) -> R<InitElement> {
    alt(
        node,
        &[
            &|n| literal(n, parse_name).map(InitElement::new_literal),
            // :timed-initial-literals
            &|n| {
                prefix(n, "at", |args| {
                    Ok((
                        parse_number(args.next()?)?,
                        literal(args.next()?, parse_name)?,
                    ))
                })
                .map(|(time, literal)| InitElement::new_at(time, literal))
            },
            // :numeric-fluents
            &|n| {
                prefix(n, "=", |args| {
                    Ok((
                        parse_basic_function_term(args.next()?)?,
                        parse_number(args.next()?)?,
                    ))
                })
                .map(|(function, value)| InitElement::new_is_value(function, value))
            },
            // :object-fluents
            &|n| {
                prefix(n, "=", |args| {
                    Ok((
                        parse_basic_function_term(args.next()?)?,
                        parse_name(args.next()?)?,
                    ))
                })
                .map(|(function, name)| InitElement::new_is_object(function, name))
            },
        ],
    )
}

crate::parsers::impl_parser!(InitElement => parse_init_el);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(InitElement::from_str("(train-not-in-use train1)").is_value(
            InitElement::new_literal(NameLiteral::new(AtomicFormula::new_predicate(
                "train-not-in-use".into(),
                ["train1".into()]
            )))
        ));

        assert!(
            InitElement::from_str("(at 10 (train-not-in-use train2))").is_value(
                InitElement::new_at(
                    Number::from(10),
                    NameLiteral::new(AtomicFormula::new_predicate(
                        "train-not-in-use".into(),
                        ["train2".into()]
                    ))
                )
            )
        );

        assert!(InitElement::from_str("(= (battery rover) 100)").is_value(
            InitElement::new_is_value(
                BasicFunctionTerm::new("battery".into(), ["rover".into()]),
                Number::from(100)
            )
        ));

        assert!(InitElement::from_str("(= (location rover) base)").is_value(
            InitElement::new_is_object(
                BasicFunctionTerm::new("location".into(), ["rover".into()]),
                Name::from("base")
            )
        ));
    }

    #[test]
    fn doc_example_0() {
        assert!(InitElement::from_str("(train-not-in-use train1)").is_value(
            InitElement::new_literal(NameLiteral::new(AtomicFormula::new_predicate(
                "train-not-in-use".into(),
                ["train1".into()]
            )))
        ));

        assert!(
            InitElement::from_str("(at 10 (train-not-in-use train2))").is_value(
                InitElement::new_at(
                    Number::from(10),
                    NameLiteral::new(AtomicFormula::new_predicate(
                        "train-not-in-use".into(),
                        ["train2".into()]
                    ))
                )
            )
        );

        assert!(InitElement::from_str("(= (battery rover) 100)").is_value(
            InitElement::new_is_value(
                BasicFunctionTerm::new("battery".into(), ["rover".into()]),
                Number::from(100)
            )
        ));

        assert!(InitElement::from_str("(= (location rover) base)").is_value(
            InitElement::new_is_object(
                BasicFunctionTerm::new("location".into(), ["rover".into()]),
                Name::from("base")
            )
        ));
    }
}
