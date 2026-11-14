//! Provides parsers for conditional effects (`<c-effect>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    alt, parse_effect, parse_effect_condition, parse_gd, parse_p_effect, prefix, typed_variables,
};
use crate::types::{ConditionalEffect, ForallConditionalEffect, WhenConditionalEffect};

/// Parses a conditional effect (`<c-effect>`), i.e. `(forall (<typed list (variable)>) <effect>)
/// | (when <GD> <cond-effect>) | <p-effect>`.
#[allow(deprecated)]
pub(crate) fn parse_c_effect(node: &Node) -> R<ConditionalEffect> {
    alt(
        node,
        &[
            &|n| parse_forall_c_effect(n).map(ConditionalEffect::from),
            &|n| parse_when_c_effect(n).map(ConditionalEffect::from),
            &|n| parse_p_effect(n).map(ConditionalEffect::from),
        ],
    )
}

/// Parses `(forall (<typed list (variable)>) <effect>)`.
///
/// ## Requirements
/// Requires `:conditional-effects`.
pub(crate) fn parse_forall_c_effect(node: &Node) -> R<ForallConditionalEffect> {
    prefix(node, "forall", |args| {
        Ok((typed_variables(args.next()?)?, parse_effect(args.next()?)?))
    })
    .map(ForallConditionalEffect::from)
}

/// Parses `(when <GD> <cond-effect>)`.
///
/// ## Requirements
/// Requires `:conditional-effects`.
#[allow(deprecated)]
pub(crate) fn parse_when_c_effect(node: &Node) -> R<WhenConditionalEffect> {
    prefix(node, "when", |args| {
        Ok((
            parse_gd(args.next()?)?,
            parse_effect_condition(args.next()?)?,
        ))
    })
    .map(WhenConditionalEffect::from)
}

crate::parsers::impl_parser!(
    ConditionalEffect => parse_c_effect,
    ForallConditionalEffect => parse_forall_c_effect,
    WhenConditionalEffect => parse_when_c_effect,
);

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let value = ConditionalEffect::from_str("(= x y)").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_primitive_effect(PrimitiveEffect::from_str("(= x y)").unwrap())
        );

        let value = ConditionalEffect::from_str("(not (= ?a B))").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_primitive_effect(
                PrimitiveEffect::from_str("(not (= ?a B))").unwrap()
            )
        );

        let value = ConditionalEffect::from_str("(forall (?a ?b) (= ?a ?b))").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_forall(
                TypedList::from_iter([
                    Typed::new_object(Variable::new_string("a")),
                    Typed::new_object(Variable::new_string("b")),
                ]),
                Effects::from_str("(= ?a ?b)").unwrap()
            )
        );

        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;
        let value = ConditionalEffect::from_str(input).unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_when(
                GoalDescription::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))")
                    .unwrap(),
                EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
            )
        );
    }

    #[test]
    fn test_parse_forall() {
        let value = ForallConditionalEffect::from_str("(forall (?a ?b) (= ?a ?b))").unwrap();
        assert_eq!(
            value,
            ForallConditionalEffect::new(
                TypedList::from_iter([
                    Typed::new_object(Variable::new_string("a")),
                    Typed::new_object(Variable::new_string("b")),
                ]),
                Effects::from_str("(= ?a ?b)").unwrap()
            )
        );
    }

    #[test]
    fn test_parse_when() {
        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;

        let value = WhenConditionalEffect::from_str(input).unwrap();
        assert_eq!(
            value,
            WhenConditionalEffect::new(
                GoalDescription::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))")
                    .unwrap(),
                EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            ConditionalEffect::from_str("(= x y)").is_value(ConditionalEffect::Effect(
                PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))
            ))
        );
        assert!(
            ConditionalEffect::from_str("(not (= ?a B))").is_value(ConditionalEffect::Effect(
                PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Variable("a".into()), Term::Name("B".into()))
                ))
            ))
        );

        assert!(
            ConditionalEffect::from_str("(forall (?a ?b) (= ?a ?b))").is_value(
                ConditionalEffect::forall(
                    TypedList::from_iter([
                        Typed::object(Variable::new_string("a")),
                        Typed::object(Variable::new_string("b")),
                    ]),
                    Effects::new(ConditionalEffect::Effect(PrimitiveEffect::AtomicFormula(
                        AtomicFormula::Equality(EqualityAtomicFormula::new(
                            Term::Variable("a".into()),
                            Term::Variable("b".into())
                        ))
                    )))
                )
            )
        );

        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;
        assert!(
            ConditionalEffect::from_str(input).is_value(ConditionalEffect::when(
                GoalDescription::new_and([
                    GoalDescription::new_atomic_formula(AtomicFormula::new_predicate(
                        Predicate::new("has-hot-chocolate".into()),
                        [Term::Variable("p".into()), Term::Variable("c".into())]
                    )),
                    GoalDescription::new_atomic_formula(AtomicFormula::new_predicate(
                        Predicate::new("has-marshmallows".into()),
                        [Term::Variable("c".into())]
                    )),
                ]),
                EffectCondition::from_iter([PrimitiveEffect::new(AtomicFormula::new_predicate(
                    Predicate::new("person-is-happy".into()),
                    [Term::Variable("p".into())]
                ))])
            ))
        );
    }

    #[test]
    fn doc_example_1() {
        assert!(
            ForallConditionalEffect::from_str("(forall (?a ?b) (= ?a ?b))").is_value(
                ForallConditionalEffect::new(
                    TypedList::from_iter([
                        Typed::object(Variable::new_string("a")),
                        Typed::object(Variable::new_string("b")),
                    ]),
                    Effects::new(ConditionalEffect::Effect(PrimitiveEffect::AtomicFormula(
                        AtomicFormula::Equality(EqualityAtomicFormula::new(
                            Term::Variable("a".into()),
                            Term::Variable("b".into())
                        ))
                    )))
                )
            )
        );
    }

    #[test]
    fn doc_example_2() {
        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;

        assert!(
            WhenConditionalEffect::from_str(input).is_value(WhenConditionalEffect::new(
                GoalDescription::new_and([
                    GoalDescription::new_atomic_formula(AtomicFormula::new_predicate(
                        Predicate::new("has-hot-chocolate".into()),
                        [Term::Variable("p".into()), Term::Variable("c".into())]
                    )),
                    GoalDescription::new_atomic_formula(AtomicFormula::new_predicate(
                        Predicate::new("has-marshmallows".into()),
                        [Term::Variable("c".into())]
                    )),
                ]),
                EffectCondition::from_iter([PrimitiveEffect::new(AtomicFormula::new_predicate(
                    Predicate::new("person-is-happy".into()),
                    [Term::Variable("p".into())]
                ))])
            ))
        );
    }

    #[test]
    fn doc_example_3() {
        let value = ConditionalEffect::from_str("(= x y)").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_primitive_effect(PrimitiveEffect::from_str("(= x y)").unwrap())
        );

        let value = ConditionalEffect::from_str("(not (= ?a B))").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_primitive_effect(
                PrimitiveEffect::from_str("(not (= ?a B))").unwrap()
            )
        );

        let value = ConditionalEffect::from_str("(forall (?a ?b) (= ?a ?b))").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::forall(
                TypedList::from_iter([
                    Typed::object(Variable::new_string("a")),
                    Typed::object(Variable::new_string("b")),
                ]),
                Effects::from_str("(= ?a ?b)").unwrap()
            )
        );

        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;
        let value = ConditionalEffect::from_str(input).unwrap();
        assert_eq!(
            value,
            ConditionalEffect::when(
                GoalDescription::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))")
                    .unwrap(),
                EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
            )
        );
    }

    #[test]
    fn doc_example_4() {
        let value = ForallConditionalEffect::from_str("(forall (?a ?b) (= ?a ?b))").unwrap();
        assert_eq!(
            value,
            ForallConditionalEffect::new(
                TypedList::from_iter([
                    Typed::object(Variable::new_string("a")),
                    Typed::object(Variable::new_string("b")),
                ]),
                Effects::from_str("(= ?a ?b)").unwrap()
            )
        );
    }

    #[test]
    fn doc_example_5() {
        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;

        let value = WhenConditionalEffect::from_str(input).unwrap();
        assert_eq!(
            value,
            WhenConditionalEffect::new(
                GoalDescription::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))")
                    .unwrap(),
                EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
            )
        );
    }
}
