//! Provides parsers for durative action effects (`<da-effect>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_da_gd, parse_timed_effect, prefix, typed_variables};
use crate::types::DurativeActionEffect;

/// Parses a durative action effect (`<da-effect>`), i.e. `<timed-effect> | (and <da-effect>*)
/// | (forall (<typed list (variable)>) <da-effect>) | (when <da-GD> <timed-effect>)`.
pub(crate) fn parse_da_effect(node: &Node) -> R<DurativeActionEffect> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "and", |args| args.list0(parse_da_effect))
                    .map(DurativeActionEffect::from_iter)
            },
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((
                        typed_variables(args.next()?)?,
                        parse_da_effect(args.next()?)?,
                    ))
                })
                .map(DurativeActionEffect::from)
            },
            &|n| {
                prefix(n, "when", |args| {
                    Ok((
                        parse_da_gd(args.next()?)?,
                        parse_timed_effect(args.next()?)?,
                    ))
                })
                .map(DurativeActionEffect::from)
            },
            &|n| parse_timed_effect(n).map(DurativeActionEffect::from),
        ],
    )
}

crate::parsers::impl_parser!(DurativeActionEffect => parse_da_effect);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn simple_works() {
        let input = r"(and
                (decrease (fuel-level ?t) (* 2 #t))
                (at end (at ?rover ?to-waypoint))
                (at end (been-at ?rover ?to-waypoint))
                (at start (not (at ?rover ?from-waypoint)))
                (at start (decrease (battery-amount ?rover) 8))
                (at end (increase (distance-travelled) 5))
                )";

        let _effect = DurativeActionEffect::from_str(input).unwrap();
    }

    #[test]
    fn complex_works() {
        let input = r"(and
                (decrease (fuel-level ?t) (* 2 #t))
                (at end (at ?rover ?to-waypoint))
                (at end (been-at ?rover ?to-waypoint))
                (at start (not (at ?rover ?from-waypoint)))
                (at start (decrease (battery-amount ?rover) 8))
                (at end (increase (distance-travelled) 5))
                )";

        let _effect = DurativeActionEffect::from_str(input).unwrap();
    }

    #[test]
    fn test_at_start() {
        check!(
            DurativeActionEffect::from_str("(at start (= x y))").is_value(
                DurativeActionEffect::Timed(TimedEffect::conditional(
                    TimeSpecifier::Start,
                    EffectCondition::new(PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                        EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                    )))
                ))
            )
        );
    }

    #[test]
    fn test_and_empty() {
        check!(DurativeActionEffect::from_str("(and )").is_value(DurativeActionEffect::and([])));
    }

    #[test]
    fn test_and() {
        check!(
            DurativeActionEffect::from_str("(and (at start (= x y)) (and ))").is_value(
                DurativeActionEffect::and([
                    DurativeActionEffect::Timed(TimedEffect::conditional(
                        TimeSpecifier::Start,
                        EffectCondition::new(PrimitiveEffect::AtomicFormula(
                            AtomicFormula::Equality(EqualityAtomicFormula::new(
                                Term::Name("x".into()),
                                Term::Name("y".into())
                            ))
                        ))
                    )),
                    DurativeActionEffect::and([])
                ])
            )
        );
    }

    #[test]
    fn test_forall() {
        check!(
            DurativeActionEffect::from_str("(forall (?a ?b) (at start (= a b)))").is_value(
                DurativeActionEffect::forall(
                    TypedList::from_iter([
                        Typed::object(Variable::new_string("a")),
                        Typed::object(Variable::new_string("b")),
                    ]),
                    DurativeActionEffect::Timed(TimedEffect::conditional(
                        TimeSpecifier::Start,
                        EffectCondition::new(PrimitiveEffect::AtomicFormula(
                            AtomicFormula::Equality(EqualityAtomicFormula::new(
                                Term::Name("a".into()),
                                Term::Name("b".into())
                            ))
                        ))
                    ))
                )
            )
        );
    }

    #[test]
    fn doc_example_0() {
        check!(
            DurativeActionEffect::from_str("(at start (= x y))").is_value(
                DurativeActionEffect::Timed(TimedEffect::conditional(
                    TimeSpecifier::Start,
                    EffectCondition::new(PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                        EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                    )))
                ))
            )
        );

        check!(DurativeActionEffect::from_str("(and )").is_value(DurativeActionEffect::new_and([])));

        check!(
            DurativeActionEffect::from_str("(and (at start (= x y)) (and ))").is_value(
                DurativeActionEffect::new_and([
                    DurativeActionEffect::Timed(TimedEffect::conditional(
                        TimeSpecifier::Start,
                        EffectCondition::new(PrimitiveEffect::AtomicFormula(
                            AtomicFormula::Equality(EqualityAtomicFormula::new(
                                Term::Name("x".into()),
                                Term::Name("y".into())
                            ))
                        ))
                    )),
                    DurativeActionEffect::new_and([])
                ])
            )
        );

        check!(
            DurativeActionEffect::from_str("(forall (?a ?b) (at start (= a b)))").is_value(
                DurativeActionEffect::new_forall(
                    TypedList::from_iter([
                        Typed::object(Variable::new_string("a")),
                        Typed::object(Variable::new_string("b")),
                    ]),
                    DurativeActionEffect::Timed(TimedEffect::conditional(
                        TimeSpecifier::Start,
                        EffectCondition::new(PrimitiveEffect::AtomicFormula(
                            AtomicFormula::Equality(EqualityAtomicFormula::new(
                                Term::Name("a".into()),
                                Term::Name("b".into())
                            ))
                        ))
                    ))
                )
            )
        );
    }
}
