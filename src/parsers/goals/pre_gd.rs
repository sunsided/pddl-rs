//! Provides parsers for precondition goal descriptions (`<pre-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_pref_gd, prefix, typed_variables};
use crate::types::{PreconditionGoalDescription, PreconditionGoalDescriptions};

/// Parses precondition goal descriptions (`<pre-GD>`), i.e. `<pref-GD> | (and <pre-GD>*)
/// | (forall (<typed list (variable)>) <pre-GD>)`.
pub(crate) fn parse_pre_gd(node: &Node) -> R<PreconditionGoalDescriptions> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((typed_variables(args.next()?)?, parse_pre_gd(args.next()?)?))
                })
                .map(|(variables, gd)| {
                    PreconditionGoalDescriptions::from(PreconditionGoalDescription::new_forall(
                        variables, gd,
                    ))
                })
            },
            &|n| {
                prefix(n, "and", |args| args.list0(parse_pre_gd))
                    .map(|x| PreconditionGoalDescriptions::from_iter(x.into_iter().flatten()))
            },
            &|n| {
                parse_pref_gd(n).map(|x| {
                    PreconditionGoalDescriptions::from(PreconditionGoalDescription::new_preference(
                        x,
                    ))
                })
            },
        ],
    )
}

crate::parsers::impl_parser!(PreconditionGoalDescriptions => parse_pre_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(PreconditionGoalDescriptions::from_str("(= x y)").is_value(
            PreconditionGoalDescriptions::new_preference(PreferenceGoalDescription::Goal(
                GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        ));

        assert!(
            PreconditionGoalDescriptions::from_str("(and (= x y) (= a b))").is_value(
                PreconditionGoalDescriptions::from_iter([
                    PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )),
                    PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                ])
            )
        );

        assert!(
            PreconditionGoalDescriptions::from_str("(forall (?a ?b) (= a b))").is_value(
                PreconditionGoalDescription::new_forall(
                    TypedList::from_iter([
                        Typed::new(Variable::new_string("a"), Type::OBJECT),
                        Typed::new(Variable::new_string("b"), Type::OBJECT),
                    ]),
                    PreconditionGoalDescription::new_preference(PreferenceGoalDescription::Goal(
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                    .into()
                )
                .into()
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(PreconditionGoalDescriptions::from_str("(= x y)").is_value(
            PreconditionGoalDescriptions::new_preference(PreferenceGoalDescription::Goal(
                GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        ));

        assert!(
            PreconditionGoalDescriptions::from_str("(and (= x y) (= a b))").is_value(
                PreconditionGoalDescriptions::from_iter([
                    PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )),
                    PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                ])
            )
        );

        assert!(
            PreconditionGoalDescriptions::from_str("(forall (?a ?b) (= a b))").is_value(
                PreconditionGoalDescription::new_forall(
                    TypedList::from_iter([
                        Typed::new(Variable::new_string("a"), Type::OBJECT),
                        Typed::new(Variable::new_string("b"), Type::OBJECT),
                    ]),
                    PreconditionGoalDescription::new_preference(PreferenceGoalDescription::Goal(
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                    .into()
                )
                .into()
            )
        );
    }
}
