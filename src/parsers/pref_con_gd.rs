//! Provides parsers for preference constraint goal definitions (`<pref-con-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_con_gd, parse_pref_name, prefix, typed_variables};
use crate::types::PreferenceConstraintGoalDefinitions;

/// Parses preference constraint goal definitions (`<pref-con-GD>`), i.e. `(and <pref-con-GD>*)
/// | (forall (<typed list (variable)>) <pref-con-GD>) | (preference [<pref-name>] <con-GD>)
/// | <con-GD>`.
pub(crate) fn parse_pref_con_gd(node: &Node) -> R<PreferenceConstraintGoalDefinitions> {
    type P = PreferenceConstraintGoalDefinitions;
    alt(
        node,
        &[
            &|n| {
                prefix(n, "and", |args| args.list0(parse_pref_con_gd))
                    .map(|x| P::from_iter(x.into_iter().flatten()))
            },
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((
                        typed_variables(args.next()?)?,
                        parse_pref_con_gd(args.next()?)?,
                    ))
                })
                .map(|(variables, gd)| P::forall(variables, gd))
            },
            &|n| {
                prefix(n, "preference", |args| {
                    Ok((parse_pref_name(args.next()?)?, parse_con_gd(args.next()?)?))
                })
                .map(|(name, gd)| P::preference(Some(name), gd))
            },
            &|n| {
                prefix(n, "preference", |args| parse_con_gd(args.next()?))
                    .map(|gd| P::preference(None, gd))
            },
            &|n| parse_con_gd(n).map(P::new_goal),
        ],
    )
}

crate::parsers::impl_parser!(PreferenceConstraintGoalDefinitions => parse_pref_con_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
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

        assert!(PreferenceConstraintGoalDefinitions::from_str("(and)")
            .is_value(PreferenceConstraintGoalDefinitions::default()));

        assert!(PreferenceConstraintGoalDefinitions::from_str(
            "(and (at end (= x y)) (at end (not (= x z))))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::from_iter([
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::at_end(
                gd_a.clone()
            )),
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::at_end(
                gd_b.clone()
            )),
        ])));

        assert!(PreferenceConstraintGoalDefinitions::from_str(
            "(forall (?x ?z) (sometime (= ?x ?z)))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::forall(
            TypedList::from_iter([
                Variable::from("x").to_typed(Type::OBJECT),
                Variable::from("z").to_typed(Type::OBJECT),
            ]),
            PreferenceConstraintGoalDefinitions::goal(ConstraintGoalDefinition::sometime(
                ConstraintGoalDefinitionInner::Goal(GoalDefinition::atomic_formula(
                    AtomicFormula::equality(Term::Variable("x".into()), Term::Variable("z".into()))
                ))
            ))
        )));

        assert!(
            PreferenceConstraintGoalDefinitions::from_str("(at end (= x y))").is_value(
                PreferenceConstraintGoalDefinitions::goal(ConstraintGoalDefinition::AtEnd(
                    gd_a.clone()
                ))
            )
        );

        assert!(
            PreferenceConstraintGoalDefinitions::from_str("(preference (at end (= x y)))")
                .is_value(PreferenceConstraintGoalDefinitions::preference(
                    None,
                    ConstraintGoalDefinition::AtEnd(gd_a.clone())
                ))
        );

        assert!(PreferenceConstraintGoalDefinitions::from_str(
            "(preference name (at end (= x y)))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::preference(
            Some("name".into()),
            ConstraintGoalDefinition::AtEnd(gd_a.clone())
        )));
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

        assert!(PreferenceConstraintGoalDefinitions::from_str("(and)")
            .is_value(PreferenceConstraintGoalDefinitions::default()));

        assert!(PreferenceConstraintGoalDefinitions::from_str(
            "(and (at end (= x y)) (at end (not (= x z))))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::from_iter([
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::at_end(
                gd_a.clone()
            )),
            PreferenceConstraintGoalDefinition::goal(ConstraintGoalDefinition::at_end(
                gd_b.clone()
            )),
        ])));

        assert!(PreferenceConstraintGoalDefinitions::from_str(
            "(forall (?x ?z) (sometime (= ?x ?z)))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::forall(
            TypedList::from_iter([
                Variable::from("x").to_typed(Type::OBJECT),
                Variable::from("z").to_typed(Type::OBJECT),
            ]),
            PreferenceConstraintGoalDefinitions::goal(ConstraintGoalDefinition::sometime(
                // gd ...
                ConstraintGoalDefinitionInner::Goal(GoalDefinition::atomic_formula(
                    AtomicFormula::equality(Term::Variable("x".into()), Term::Variable("z".into()))
                ))
            ))
        )));

        assert!(
            PreferenceConstraintGoalDefinitions::from_str("(at end (= x y))").is_value(
                PreferenceConstraintGoalDefinitions::goal(ConstraintGoalDefinition::AtEnd(
                    gd_a.clone()
                ))
            )
        );

        assert!(
            PreferenceConstraintGoalDefinitions::from_str("(preference (at end (= x y)))")
                .is_value(PreferenceConstraintGoalDefinitions::preference(
                    None,
                    ConstraintGoalDefinition::AtEnd(gd_a.clone())
                ))
        );

        assert!(PreferenceConstraintGoalDefinitions::from_str(
            "(preference name (at end (= x y)))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::preference(
            Some("name".into()),
            ConstraintGoalDefinition::AtEnd(gd_a.clone())
        )));
    }
}
