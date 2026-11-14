//! Provides parsers for effect conditions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_p_effect, prefix};
use crate::types::EffectCondition;

/// Parses an effect condition, i.e. `(and <p-effect>*) | <p-effect>`.
#[allow(deprecated)]
pub(crate) fn parse_effect_condition(node: &Node) -> R<EffectCondition> {
    alt(
        node,
        &[
            &|n| prefix(n, "and", |args| args.list0(parse_p_effect)).map(EffectCondition::from),
            &|n| parse_p_effect(n).map(EffectCondition::from),
        ],
    )
}

crate::parsers::impl_parser!(EffectCondition => parse_effect_condition);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            EffectCondition::from_str("(= x y)").is_value(EffectCondition::Single(
                PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))
            ))
        );

        assert!(
            EffectCondition::from_str("(and (= x y) (not (= ?a B)))").is_value(
                EffectCondition::All(vec![
                    PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                        EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                    )),
                    PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(
                        EqualityAtomicFormula::new(
                            Term::Variable("a".into()),
                            Term::Name("B".into())
                        )
                    ))
                ])
            )
        );
    }

    #[test]
    fn test_parse_empty_and() {
        assert!(EffectCondition::from_str("(and)").is_value(EffectCondition::All(vec![])));
    }

    #[test]
    fn doc_example_0() {
        assert!(
            EffectCondition::from_str("(= x y)").is_value(EffectCondition::Single(
                PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))
            ))
        );

        assert!(
            EffectCondition::from_str("(and (= x y) (not (= ?a B)))").is_value(
                EffectCondition::All(vec![
                    PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                        EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                    )),
                    PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(
                        EqualityAtomicFormula::new(
                            Term::Variable("a".into()),
                            Term::Name("B".into())
                        )
                    ))
                ])
            )
        );
    }
}
