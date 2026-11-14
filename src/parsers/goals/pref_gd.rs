//! Provides parsers for preference goal descriptions (`<pref-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_gd, parse_pref_name, prefix};
use crate::types::{Preference, PreferenceGoalDescription};

/// Parses a preference goal description (`<pref-GD>`), i.e. `<GD> | (preference [<pref-name>] <GD>)`.
pub(crate) fn parse_pref_gd(node: &Node) -> R<PreferenceGoalDescription> {
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
                .map(PreferenceGoalDescription::from_preference)
            },
            &|n| parse_gd(n).map(PreferenceGoalDescription::from_gd),
        ],
    )
}

crate::parsers::impl_parser!(PreferenceGoalDescription => parse_pref_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    /// `preference` must match as a whole word, not as the start of a predicate name.
    #[test]
    fn predicate_starting_with_preference_is_not_a_preference() {
        for input in ["(preferences(p))", "(preferences (p))"] {
            let value = PreferenceGoalDescription::from_str(input).unwrap();
            let PreferenceGoalDescription::Goal(GoalDescription::AtomicFormula(formula)) = &value
            else {
                panic!("{input}: expected an atomic formula, got {value:?}");
            };
            check!(
                format!("{formula:?}").contains("preferences"),
                "{input}: {formula:?}"
            );
        }
    }

    #[test]
    fn unnamed_preference_without_whitespace() {
        let value = PreferenceGoalDescription::from_str("(preference(p))").unwrap();
        let PreferenceGoalDescription::Preference(pref) = &value else {
            panic!("expected a preference, got {value:?}");
        };
        check!(pref == &Preference::new(None, GoalDescription::from_str("(p)").unwrap()));
    }

    #[test]
    fn test_parse() {
        // Simple goal description.
        check!(PreferenceGoalDescription::from_str("(= x y)").is_value(
            PreferenceGoalDescription::Goal(GoalDescription::AtomicFormula(
                AtomicFormula::new_equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        ));

        // Named preference.
        check!(
            PreferenceGoalDescription::from_str("(preference p (= x y))").is_value(
                PreferenceGoalDescription::Preference(Preference::new(
                    Some(PreferenceName::from("p")),
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        // Unnamed preference.
        check!(
            PreferenceGoalDescription::from_str("(preference (= x y))").is_value(
                PreferenceGoalDescription::Preference(Preference::new(
                    None,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );
    }

    #[test]
    fn doc_example_0() {
        // Simple goal description.
        check!(PreferenceGoalDescription::from_str("(= x y)").is_value(
            PreferenceGoalDescription::Goal(GoalDescription::AtomicFormula(
                AtomicFormula::new_equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        ));

        // Named preference.
        check!(
            PreferenceGoalDescription::from_str("(preference p (= x y))").is_value(
                PreferenceGoalDescription::Preference(Preference::new(
                    Some(PreferenceName::from("p")),
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        // Unnamed preference.
        check!(
            PreferenceGoalDescription::from_str("(preference (= x y))").is_value(
                PreferenceGoalDescription::Preference(Preference::new(
                    None,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );
    }
}
