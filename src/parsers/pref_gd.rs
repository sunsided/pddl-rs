//! Provides parsers for preference goal definitions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_gd, parse_pref_name, prefix};
use crate::types::{Preference, PreferenceGoalDefinition};

/// Parses a preference goal definition, i.e. `<GD> | (preference [<pref-name>] <GD>)`.
pub(crate) fn parse_pref_gd(node: &Node) -> R<PreferenceGoalDefinition> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "preference", |args| {
                    let name = args.peek().and_then(|n| parse_pref_name(n).ok());
                    if name.is_some() {
                        args.next()?;
                    }
                    Ok(Preference::new(name, parse_gd(args.next()?)?))
                })
                .map(PreferenceGoalDefinition::from_preference)
            },
            &|n| parse_gd(n).map(PreferenceGoalDefinition::from_gd),
        ],
    )
}

crate::parsers::impl_parser!(PreferenceGoalDefinition => parse_pref_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    /// `preference` must match as a whole word, not as the start of a predicate name.
    #[test]
    fn predicate_starting_with_preference_is_not_a_preference() {
        for input in ["(preferences(p))", "(preferences (p))"] {
            let value = PreferenceGoalDefinition::from_str(input).unwrap();
            let PreferenceGoalDefinition::Goal(GoalDefinition::AtomicFormula(formula)) = &value
            else {
                panic!("{input}: expected an atomic formula, got {value:?}");
            };
            assert!(
                format!("{formula:?}").contains("preferences"),
                "{input}: {formula:?}"
            );
        }
    }

    #[test]
    fn unnamed_preference_without_whitespace() {
        let value = PreferenceGoalDefinition::from_str("(preference(p))").unwrap();
        let PreferenceGoalDefinition::Preference(pref) = &value else {
            panic!("expected a preference, got {value:?}");
        };
        assert_eq!(
            pref,
            &Preference::new(None, GoalDefinition::from_str("(p)").unwrap())
        );
    }

    #[test]
    fn test_parse() {
        // Simple goal definition.
        assert!(PreferenceGoalDefinition::from_str("(= x y)").is_value(
            PreferenceGoalDefinition::Goal(GoalDefinition::AtomicFormula(
                AtomicFormula::new_equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        ));

        // Named preference.
        assert!(
            PreferenceGoalDefinition::from_str("(preference p (= x y))").is_value(
                PreferenceGoalDefinition::Preference(Preference::new(
                    Some(PreferenceName::from("p")),
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        // Unnamed preference.
        assert!(
            PreferenceGoalDefinition::from_str("(preference (= x y))").is_value(
                PreferenceGoalDefinition::Preference(Preference::new(
                    None,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );
    }

    #[test]
    fn doc_example_0() {
        // Simple goal definition.
        assert!(PreferenceGoalDefinition::from_str("(= x y)").is_value(
            PreferenceGoalDefinition::Goal(GoalDefinition::AtomicFormula(
                AtomicFormula::new_equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        ));

        // Named preference.
        assert!(
            PreferenceGoalDefinition::from_str("(preference p (= x y))").is_value(
                PreferenceGoalDefinition::Preference(Preference::new(
                    Some(PreferenceName::from("p")),
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        // Unnamed preference.
        assert!(
            PreferenceGoalDefinition::from_str("(preference (= x y))").is_value(
                PreferenceGoalDefinition::Preference(Preference::new(
                    None,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );
    }
}
