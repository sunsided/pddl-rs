//! Provides parsers for durative action goal definitions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_pref_timed_gd, prefix, typed_variables};
use crate::types::DurativeActionGoalDefinition;

/// Parses a durative action goal definition, i.e. `<pref-timed-GD> | (and <da-GD>*)
/// | (forall (<typed list (variable)>) <da-GD>)`.
pub(crate) fn parse_da_gd(node: &Node) -> R<DurativeActionGoalDefinition> {
    type G = DurativeActionGoalDefinition;
    alt(
        node,
        &[
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((typed_variables(args.next()?)?, parse_da_gd(args.next()?)?))
                })
                .map(|(variables, gd)| G::forall(variables, gd))
            },
            &|n| prefix(n, "and", |args| args.list0(parse_da_gd)).map(G::new_and),
            &|n| parse_pref_timed_gd(n).map(G::new_timed),
        ],
    )
}

crate::parsers::impl_parser!(DurativeActionGoalDefinition => parse_da_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn complex_works() {
        let input = r#"(and
                (at start (rover ?rover))
                (at start (waypoint ?from-waypoint))
                (at start (waypoint ?to-waypoint))
                (over all (can-move ?from-waypoint ?to-waypoint))
                (at start (at ?rover ?from-waypoint))
                (at start (> (battery-amount ?rover) 8)))"#;
        let _gd = DurativeActionGoalDefinition::from_str(input).unwrap();
    }

    #[test]
    fn test_at_start() {
        assert!(
            DurativeActionGoalDefinition::from_str("(at start (= x y))").is_value(
                DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                    TimedGoalDefinition::at(
                        TimeSpecifier::Start,
                        GoalDefinition::AtomicFormula(AtomicFormula::equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                ))
            )
        );
    }

    #[test]
    fn test_and_empty() {
        assert!(DurativeActionGoalDefinition::from_str("(and )")
            .is_value(DurativeActionGoalDefinition::and([])));
    }

    #[test]
    fn test_and() {
        assert!(DurativeActionGoalDefinition::from_str(
            "(and (at start (= x y)) (over all (= a b)))"
        )
        .is_value(DurativeActionGoalDefinition::and([
            DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                TimedGoalDefinition::at(
                    TimeSpecifier::Start,
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            )),
            DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                TimedGoalDefinition::over(
                    Interval::All,
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("a".into()),
                        Term::Name("b".into())
                    ))
                )
            ))
        ])));
    }

    #[test]
    fn test_forall() {
        assert!(
            DurativeActionGoalDefinition::from_str("(forall (?a ?b) (at start (= a b)))").is_value(
                DurativeActionGoalDefinition::forall(
                    TypedList::from_iter([
                        Typed::object(Variable::string("a")),
                        Typed::object(Variable::string("b")),
                    ]),
                    DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                        TimedGoalDefinition::at(
                            TimeSpecifier::Start,
                            GoalDefinition::AtomicFormula(AtomicFormula::equality(
                                Term::Name("a".into()),
                                Term::Name("b".into())
                            ))
                        )
                    ))
                )
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            DurativeActionGoalDefinition::from_str("(at start (= x y))").is_value(
                DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                    TimedGoalDefinition::at(
                        TimeSpecifier::Start,
                        GoalDefinition::AtomicFormula(AtomicFormula::equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                ))
            )
        );

        assert!(DurativeActionGoalDefinition::from_str("(and )")
            .is_value(DurativeActionGoalDefinition::and([])));

        assert!(DurativeActionGoalDefinition::from_str(
            "(and (at start (= x y)) (over all (= a b)))"
        )
        .is_value(DurativeActionGoalDefinition::and([
            DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                TimedGoalDefinition::at(
                    TimeSpecifier::Start,
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            )),
            DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                TimedGoalDefinition::over(
                    Interval::All,
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("a".into()),
                        Term::Name("b".into())
                    ))
                )
            ))
        ])));

        assert!(
            DurativeActionGoalDefinition::from_str("(forall (?a ?b) (at start (= a b)))").is_value(
                DurativeActionGoalDefinition::forall(
                    TypedList::from_iter([
                        Typed::object(Variable::string("a")),
                        Typed::object(Variable::string("b")),
                    ]),
                    DurativeActionGoalDefinition::Timed(PreferenceTimedGoalDefinition::Required(
                        TimedGoalDefinition::at(
                            TimeSpecifier::Start,
                            GoalDefinition::AtomicFormula(AtomicFormula::equality(
                                Term::Name("a".into()),
                                Term::Name("b".into())
                            ))
                        )
                    ))
                )
            )
        );
    }
}
