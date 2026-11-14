//! Provides parsers for effects.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_c_effect, prefix};
use crate::types::Effects;

/// Parses effects, i.e. `(and <c-effect>*) | <c-effect>`.
pub(crate) fn parse_effect(node: &Node) -> R<Effects> {
    alt(
        node,
        &[
            &|n| prefix(n, "and", |args| args.list0(parse_c_effect)).map(Effects::from),
            &|n| parse_c_effect(n).map(Effects::from),
        ],
    )
}

crate::parsers::impl_parser!(Effects => parse_effect);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            Effects::from_str("(= x y)").is_value(Effects::new(ConditionalEffect::Effect(
                PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))
            )))
        );
        assert!(
            Effects::from_str("(and (= x y) (not (= ?a B)))").is_value(Effects::from_iter([
                ConditionalEffect::Effect(PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))),
                ConditionalEffect::Effect(PrimitiveEffect::NotAtomicFormula(
                    AtomicFormula::Equality(EqualityAtomicFormula::new(
                        Term::Variable("a".into()),
                        Term::Name("B".into())
                    ))
                ))
            ]))
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            Effects::from_str("(= x y)").is_value(Effects::new(ConditionalEffect::Effect(
                PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))
            )))
        );
        assert!(
            Effects::from_str("(and (= x y) (not (= ?a B)))").is_value(Effects::from_iter([
                ConditionalEffect::Effect(PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))),
                ConditionalEffect::Effect(PrimitiveEffect::NotAtomicFormula(
                    AtomicFormula::Equality(EqualityAtomicFormula::new(
                        Term::Variable("a".into()),
                        Term::Name("B".into())
                    ))
                ))
            ]))
        );
    }
}
