//! Provides parsers for preference constraint goal descriptions (`<pref-con-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_con_gd, parse_pref_name, prefix, typed_variables};
use crate::types::PreferenceConstraintGoalDescriptions;

/// Parses preference constraint goal descriptions (`<pref-con-GD>`), i.e. `(and <pref-con-GD>*)
/// | (forall (<typed list (variable)>) <pref-con-GD>) | (preference [<pref-name>] <con-GD>)
/// | <con-GD>`.
pub(crate) fn parse_pref_con_gd(node: &Node) -> R<PreferenceConstraintGoalDescriptions> {
    type P = PreferenceConstraintGoalDescriptions;
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

crate::parsers::impl_parser!(PreferenceConstraintGoalDescriptions => parse_pref_con_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        // (= x y)
        let gd_a = GoalDescription::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));

        // (not (= x z))
        let gd_b = GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("z".into()),
        )));

        assert!(PreferenceConstraintGoalDescriptions::from_str("(and)")
            .is_value(PreferenceConstraintGoalDescriptions::default()));

        assert!(PreferenceConstraintGoalDescriptions::from_str(
            "(and (at end (= x y)) (at end (not (= x z))))"
        )
        .is_value(PreferenceConstraintGoalDescriptions::from_iter([
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::at_end(
                gd_a.clone()
            )),
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::at_end(
                gd_b.clone()
            )),
        ])));

        assert!(PreferenceConstraintGoalDescriptions::from_str(
            "(forall (?x ?z) (sometime (= ?x ?z)))"
        )
        .is_value(PreferenceConstraintGoalDescriptions::forall(
            TypedList::from_iter([
                Variable::from("x").to_typed(Type::OBJECT),
                Variable::from("z").to_typed(Type::OBJECT),
            ]),
            PreferenceConstraintGoalDescriptions::goal(ConstraintGoalDescription::sometime(
                ConstraintGoalDescriptionInner::Goal(GoalDescription::atomic_formula(
                    AtomicFormula::equality(Term::Variable("x".into()), Term::Variable("z".into()))
                ))
            ))
        )));

        assert!(
            PreferenceConstraintGoalDescriptions::from_str("(at end (= x y))").is_value(
                PreferenceConstraintGoalDescriptions::goal(ConstraintGoalDescription::AtEnd(
                    gd_a.clone()
                ))
            )
        );

        assert!(
            PreferenceConstraintGoalDescriptions::from_str("(preference (at end (= x y)))")
                .is_value(PreferenceConstraintGoalDescriptions::preference(
                    None,
                    ConstraintGoalDescription::AtEnd(gd_a.clone())
                ))
        );

        assert!(PreferenceConstraintGoalDescriptions::from_str(
            "(preference name (at end (= x y)))"
        )
        .is_value(PreferenceConstraintGoalDescriptions::preference(
            Some("name".into()),
            ConstraintGoalDescription::AtEnd(gd_a.clone())
        )));
    }

    #[test]
    fn doc_example_0() {
        // (= x y)
        let gd_a = GoalDescription::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));

        // (not (= x z))
        let gd_b = GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("z".into()),
        )));

        assert!(PreferenceConstraintGoalDescriptions::from_str("(and)")
            .is_value(PreferenceConstraintGoalDescriptions::default()));

        assert!(PreferenceConstraintGoalDescriptions::from_str(
            "(and (at end (= x y)) (at end (not (= x z))))"
        )
        .is_value(PreferenceConstraintGoalDescriptions::from_iter([
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::at_end(
                gd_a.clone()
            )),
            PreferenceConstraintGoalDescription::goal(ConstraintGoalDescription::at_end(
                gd_b.clone()
            )),
        ])));

        assert!(PreferenceConstraintGoalDescriptions::from_str(
            "(forall (?x ?z) (sometime (= ?x ?z)))"
        )
        .is_value(PreferenceConstraintGoalDescriptions::forall(
            TypedList::from_iter([
                Variable::from("x").to_typed(Type::OBJECT),
                Variable::from("z").to_typed(Type::OBJECT),
            ]),
            PreferenceConstraintGoalDescriptions::goal(ConstraintGoalDescription::sometime(
                // gd ...
                ConstraintGoalDescriptionInner::Goal(GoalDescription::atomic_formula(
                    AtomicFormula::equality(Term::Variable("x".into()), Term::Variable("z".into()))
                ))
            ))
        )));

        assert!(
            PreferenceConstraintGoalDescriptions::from_str("(at end (= x y))").is_value(
                PreferenceConstraintGoalDescriptions::goal(ConstraintGoalDescription::AtEnd(
                    gd_a.clone()
                ))
            )
        );

        assert!(
            PreferenceConstraintGoalDescriptions::from_str("(preference (at end (= x y)))")
                .is_value(PreferenceConstraintGoalDescriptions::preference(
                    None,
                    ConstraintGoalDescription::AtEnd(gd_a.clone())
                ))
        );

        assert!(PreferenceConstraintGoalDescriptions::from_str(
            "(preference name (at end (= x y)))"
        )
        .is_value(PreferenceConstraintGoalDescriptions::preference(
            Some("name".into()),
            ConstraintGoalDescription::AtEnd(gd_a.clone())
        )));
    }
}
