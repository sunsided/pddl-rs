//! Provides parsers for precondition goal definitions (`<pre-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_pref_gd, prefix, typed_variables};
use crate::types::{PreconditionGoalDefinition, PreconditionGoalDefinitions};

/// Parses precondition goal definitions (`<pre-GD>`), i.e. `<pref-GD> | (and <pre-GD>*)
/// | (forall (<typed list (variable)>) <pre-GD>)`.
pub(crate) fn parse_pre_gd(node: &Node) -> R<PreconditionGoalDefinitions> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((typed_variables(args.next()?)?, parse_pre_gd(args.next()?)?))
                })
                .map(|(variables, gd)| {
                    PreconditionGoalDefinitions::from(PreconditionGoalDefinition::new_forall(
                        variables, gd,
                    ))
                })
            },
            &|n| {
                prefix(n, "and", |args| args.list0(parse_pre_gd))
                    .map(|x| PreconditionGoalDefinitions::from_iter(x.into_iter().flatten()))
            },
            &|n| {
                parse_pref_gd(n).map(|x| {
                    PreconditionGoalDefinitions::from(PreconditionGoalDefinition::new_preference(x))
                })
            },
        ],
    )
}

crate::parsers::impl_parser!(PreconditionGoalDefinitions => parse_pre_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(PreconditionGoalDefinitions::from_str("(= x y)").is_value(
            PreconditionGoalDefinitions::new_preference(PreferenceGoalDefinition::Goal(
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        ));

        assert!(
            PreconditionGoalDefinitions::from_str("(and (= x y) (= a b))").is_value(
                PreconditionGoalDefinitions::from_iter([
                    PreconditionGoalDefinition::Preference(PreferenceGoalDefinition::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )),
                    PreconditionGoalDefinition::Preference(PreferenceGoalDefinition::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                ])
            )
        );

        assert!(
            PreconditionGoalDefinitions::from_str("(forall (?a ?b) (= a b))").is_value(
                PreconditionGoalDefinition::new_forall(
                    TypedList::from_iter([
                        Typed::new(Variable::new_string("a"), Type::OBJECT),
                        Typed::new(Variable::new_string("b"), Type::OBJECT),
                    ]),
                    PreconditionGoalDefinition::new_preference(PreferenceGoalDefinition::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
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
        assert!(PreconditionGoalDefinitions::from_str("(= x y)").is_value(
            PreconditionGoalDefinitions::new_preference(PreferenceGoalDefinition::Goal(
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        ));

        assert!(
            PreconditionGoalDefinitions::from_str("(and (= x y) (= a b))").is_value(
                PreconditionGoalDefinitions::from_iter([
                    PreconditionGoalDefinition::Preference(PreferenceGoalDefinition::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )),
                    PreconditionGoalDefinition::Preference(PreferenceGoalDefinition::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                ])
            )
        );

        assert!(
            PreconditionGoalDefinitions::from_str("(forall (?a ?b) (= a b))").is_value(
                PreconditionGoalDefinition::new_forall(
                    TypedList::from_iter([
                        Typed::new(Variable::new_string("a"), Type::OBJECT),
                        Typed::new(Variable::new_string("b"), Type::OBJECT),
                    ]),
                    PreconditionGoalDefinition::new_preference(PreferenceGoalDefinition::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
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
