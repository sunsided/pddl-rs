//! Provides the [`literal`] rule.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, atomic_formula, prefix};
use crate::types::Literal;

/// Parses a literal (`<literal(t)>`), i.e. `<atomic formula(t)> | (not <atomic formula(t)>)`, with `inner` for each `t`.
pub(crate) fn literal<'n, 'a, O>(
    node: &'n Node<'a>,
    inner: impl Fn(&'n Node<'a>) -> R<O>,
) -> R<Literal<O>> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "not", |args| atomic_formula(args.next()?, &inner)).map(Literal::new_not)
            },
            &|n| atomic_formula(n, &inner).map(Literal::new),
        ],
    )
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::parsers::{literal, parse_name, parse_one};
    use crate::*;
    use assert2::check;

    #[test]
    fn test_parse() {
        check!(
            parse_one("(= x y)", |n| literal(n, parse_name)).is_value(Literal::AtomicFormula(
                AtomicFormula::Equality(EqualityAtomicFormula::new("x".into(), "y".into()))
            ))
        );
        check!(
            parse_one("(not (= x y))", |n| literal(n, parse_name)).is_value(
                Literal::NotAtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                    "x".into(),
                    "y".into()
                )))
            )
        );
    }

    #[test]
    fn doc_example_0() {
        check!(
            parse_one("(= x y)", |n| literal(n, parse_name)).is_value(Literal::AtomicFormula(
                AtomicFormula::Equality(EqualityAtomicFormula::new("x".into(), "y".into()))
            ))
        );
        check!(
            parse_one("(not (= x y))", |n| literal(n, parse_name)).is_value(
                Literal::NotAtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                    "x".into(),
                    "y".into()
                )))
            )
        );
    }
}
