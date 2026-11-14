//! Provides parsers for constraint goal definitions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_gd, parse_number, prefix, typed_variables};
use crate::types::{ConstraintGoalDefinition, ConstraintGoalDefinitionInner};

/// Parses a constraint goal definition, such as `(always <con2-GD>)` or `(at end <GD>)`.
///
/// ## Requirements
/// Requires `:constraints`.
pub(crate) fn parse_con_gd(node: &Node) -> R<ConstraintGoalDefinition> {
    type C = ConstraintGoalDefinition;
    alt(
        node,
        &[
            &|n| prefix(n, "and", |args| args.list0(parse_con_gd)).map(C::new_and),
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((typed_variables(args.next()?)?, parse_con_gd(args.next()?)?))
                })
                .map(|(variables, gd)| C::forall(variables, gd))
            },
            &|n| {
                prefix(n, "at", |args| {
                    args.expect("end")?;
                    parse_gd(args.next()?)
                })
                .map(C::new_at_end)
            },
            &|n| prefix(n, "always", |args| parse_con2_gd(args.next()?)).map(C::new_always),
            &|n| prefix(n, "sometime", |args| parse_con2_gd(args.next()?)).map(C::new_sometime),
            &|n| {
                prefix(n, "within", |args| {
                    Ok((parse_number(args.next()?)?, parse_con2_gd(args.next()?)?))
                })
                .map(|(number, gd)| C::within(number, gd))
            },
            &|n| {
                prefix(n, "at-most-once", |args| parse_con2_gd(args.next()?))
                    .map(C::new_at_most_once)
            },
            &|n| {
                prefix(n, "sometime-after", |args| {
                    Ok((parse_con2_gd(args.next()?)?, parse_con2_gd(args.next()?)?))
                })
                .map(|(a, b)| C::sometime_after(a, b))
            },
            &|n| {
                prefix(n, "sometime-before", |args| {
                    Ok((parse_con2_gd(args.next()?)?, parse_con2_gd(args.next()?)?))
                })
                .map(|(a, b)| C::sometime_before(a, b))
            },
            &|n| {
                prefix(n, "always-within", |args| {
                    Ok((
                        parse_number(args.next()?)?,
                        parse_con2_gd(args.next()?)?,
                        parse_con2_gd(args.next()?)?,
                    ))
                })
                .map(|(number, a, b)| C::always_within(number, a, b))
            },
            &|n| {
                prefix(n, "hold-during", |args| {
                    Ok((
                        parse_number(args.next()?)?,
                        parse_number(args.next()?)?,
                        parse_con2_gd(args.next()?)?,
                    ))
                })
                .map(|(t0, t1, gd)| C::hold_during(t0, t1, gd))
            },
            &|n| {
                prefix(n, "hold-after", |args| {
                    Ok((parse_number(args.next()?)?, parse_con2_gd(args.next()?)?))
                })
                .map(|(time, gd)| C::hold_after(time, gd))
            },
        ],
    )
}

/// Parses a `<con2-GD>`: a goal definition, or else a nested constraint goal definition.
pub(crate) fn parse_con2_gd(node: &Node) -> R<ConstraintGoalDefinitionInner> {
    alt(
        node,
        &[
            &|n| parse_gd(n).map(ConstraintGoalDefinitionInner::new_goal),
            &|n| parse_con_gd(n).map(ConstraintGoalDefinitionInner::new_nested),
        ],
    )
}

crate::parsers::impl_parser!(
    ConstraintGoalDefinition => parse_con_gd,
    ConstraintGoalDefinitionInner => parse_con2_gd,
);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let gd = GoalDefinition::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));

        let input = "(within 10 (at-most-once (= x y)))";
        assert!(ConstraintGoalDefinition::from_str(input).is_value(
            ConstraintGoalDefinition::within(
                Number::from(10),
                ConstraintGoalDefinitionInner::nested(ConstraintGoalDefinition::at_most_once(
                    ConstraintGoalDefinitionInner::goal(gd.clone())
                ))
            )
        ));

        let input = "(within 10 (at-most-once (= x y)))";
        assert!(ConstraintGoalDefinitionInner::from_str(input).is_value(
            ConstraintGoalDefinitionInner::Nested(Box::new(ConstraintGoalDefinition::within(
                Number::from(10),
                ConstraintGoalDefinitionInner::nested(ConstraintGoalDefinition::at_most_once(
                    ConstraintGoalDefinitionInner::goal(gd)
                ))
            )))
        ));
    }

    #[test]
    fn test_parse_sometime() {
        assert!(
            ConstraintGoalDefinition::from_str("(sometime (= x y))").is_value(
                ConstraintGoalDefinition::sometime(ConstraintGoalDefinitionInner::goal(
                    GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into()),
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_parse_sometime_after() {
        assert!(
            ConstraintGoalDefinition::from_str("(sometime-after (= x y) (= a b))").is_value(
                ConstraintGoalDefinition::sometime_after(
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
                    )),
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("a".into()), Term::Name("b".into()))
                    ))
                )
            )
        );
    }

    #[test]
    fn test_parse_hold_during() {
        assert!(
            ConstraintGoalDefinition::from_str("(hold-during 5 10 (= x y))").is_value(
                ConstraintGoalDefinition::hold_during(
                    Number::from(5),
                    Number::from(10),
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
                    ))
                )
            )
        );
    }

    #[test]
    fn test_parse_hold_after() {
        assert!(
            ConstraintGoalDefinition::from_str("(hold-after 5 (= x y))").is_value(
                ConstraintGoalDefinition::hold_after(
                    Number::from(5),
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
                    ))
                )
            )
        );
    }

    #[test]
    fn doc_example_0() {
        // (= x y)
        let gd_a = GoalDefinition::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));

        // (not (= x z))
        let gd_b = GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("z".into()),
        )));

        assert!(
            ConstraintGoalDefinition::from_str("(and)").is_value(ConstraintGoalDefinition::and([]))
        );

        assert!(ConstraintGoalDefinition::from_str(
            "(and (at end (= x y)) (at end (not (= x z))))"
        )
        .is_value(ConstraintGoalDefinition::and([
            ConstraintGoalDefinition::at_end(gd_a.clone()),
            ConstraintGoalDefinition::at_end(gd_b.clone()),
        ])));

        assert!(
            ConstraintGoalDefinition::from_str("(forall (?x ?z) (sometime (= ?x ?z)))").is_value(
                ConstraintGoalDefinition::forall(
                    TypedList::from_iter([
                        Variable::from("x").to_typed(Type::OBJECT),
                        Variable::from("z").to_typed(Type::OBJECT),
                    ]),
                    ConstraintGoalDefinition::sometime(ConstraintGoalDefinitionInner::Goal(
                        // gd ...
                        GoalDefinition::atomic_formula(AtomicFormula::equality(
                            Term::Variable("x".into()),
                            Term::Variable("z".into())
                        ))
                    ))
                )
            )
        );

        assert!(ConstraintGoalDefinition::from_str("(at end (= x y))")
            .is_value(ConstraintGoalDefinition::AtEnd(gd_a.clone())));

        assert!(
            ConstraintGoalDefinition::from_str("(always (= x y))").is_value(
                ConstraintGoalDefinition::Always(ConstraintGoalDefinitionInner::goal(gd_a.clone()))
            )
        );

        assert!(
            ConstraintGoalDefinition::from_str("(sometime (= x y))").is_value(
                ConstraintGoalDefinition::Sometime(ConstraintGoalDefinitionInner::goal(
                    gd_a.clone()
                ))
            )
        );

        assert!(
            ConstraintGoalDefinition::from_str("(within 10 (= x y))").is_value(
                ConstraintGoalDefinition::Within(
                    Number::from(10),
                    ConstraintGoalDefinitionInner::goal(gd_a.clone())
                )
            )
        );

        assert!(
            ConstraintGoalDefinition::from_str("(at-most-once (= x y))").is_value(
                ConstraintGoalDefinition::AtMostOnce(ConstraintGoalDefinitionInner::goal(
                    gd_a.clone()
                ))
            )
        );

        assert!(
            ConstraintGoalDefinition::from_str("(sometime-after (= x y) (not (= x z)))").is_value(
                ConstraintGoalDefinition::SometimeAfter(
                    ConstraintGoalDefinitionInner::goal(gd_a.clone()),
                    ConstraintGoalDefinitionInner::goal(gd_b.clone())
                )
            )
        );

        assert!(
            ConstraintGoalDefinition::from_str("(sometime-before (= x y) (not (= x z)))").is_value(
                ConstraintGoalDefinition::SometimeBefore(
                    ConstraintGoalDefinitionInner::goal(gd_a.clone()),
                    ConstraintGoalDefinitionInner::goal(gd_b.clone())
                )
            )
        );

        assert!(
            ConstraintGoalDefinition::from_str("(always-within 10 (= x y) (not (= x z)))")
                .is_value(ConstraintGoalDefinition::AlwaysWithin(
                    Number::from(10),
                    ConstraintGoalDefinitionInner::goal(gd_a.clone()),
                    ConstraintGoalDefinitionInner::goal(gd_b.clone())
                ))
        );

        assert!(
            ConstraintGoalDefinition::from_str("(hold-during 10 20 (= x y))").is_value(
                ConstraintGoalDefinition::HoldDuring(
                    Number::from(10),
                    Number::from(20),
                    ConstraintGoalDefinitionInner::goal(gd_a.clone())
                )
            )
        );

        assert!(
            ConstraintGoalDefinition::from_str("(hold-after 10 (= x y))").is_value(
                ConstraintGoalDefinition::HoldAfter(
                    Number::from(10),
                    ConstraintGoalDefinitionInner::goal(gd_a.clone())
                )
            )
        );
    }

    #[test]
    fn doc_example_1() {
        // (= x y)
        let gd = GoalDefinition::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));

        let input = "(within 10 (at-most-once (= x y)))";
        assert!(ConstraintGoalDefinition::from_str(input).is_value(
            ConstraintGoalDefinition::within(
                Number::from(10),
                ConstraintGoalDefinitionInner::nested(ConstraintGoalDefinition::at_most_once(
                    ConstraintGoalDefinitionInner::goal(
                        // gd ...
                        gd
                    )
                ))
            )
        ));
    }

    /// `at end` used to need exactly one space between the words.
    #[test]
    fn at_end_allows_any_whitespace_and_comments() {
        let expected = ConstraintGoalDefinition::from_str("(at end (p))").unwrap();
        for input in ["(at\n  end (p))", "(AT ; comment\nEND (p))"] {
            assert_eq!(
                ConstraintGoalDefinition::from_str(input),
                Ok(expected.clone()),
                "{input}"
            );
        }
    }
}
