//! Provides parsers for preference timed goal descriptions (`<pref-timed-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_pref_name, parse_timed_gd, prefix};
use crate::types::PreferenceTimedGoalDescription;

/// Parses a preference timed goal description (`<pref-timed-GD>`), i.e. `<timed-GD>
/// | (preference [<pref-name>] <timed-GD>)`.
pub(crate) fn parse_pref_timed_gd(node: &Node) -> R<PreferenceTimedGoalDescription> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "preference", |args| {
                    let name = args.peek().and_then(|n| parse_pref_name(n).ok());
                    if name.is_some() {
                        args.next()?;
                    }
                    Ok((name, parse_timed_gd(args.next()?)?))
                })
                .map(PreferenceTimedGoalDescription::from)
            },
            &|n| parse_timed_gd(n).map(PreferenceTimedGoalDescription::from),
        ],
    )
}

crate::parsers::impl_parser!(PreferenceTimedGoalDescription => parse_pref_timed_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            PreferenceTimedGoalDescription::from_str("(at start (= x y))").is_value(
                PreferenceTimedGoalDescription::Required(TimedGoalDescription::at(
                    TimeSpecifier::Start,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        assert!(
            PreferenceTimedGoalDescription::from_str("(preference (over all (= x y)))").is_value(
                PreferenceTimedGoalDescription::Preference(
                    None,
                    TimedGoalDescription::over(
                        Interval::All,
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                )
            )
        );

        assert!(PreferenceTimedGoalDescription::from_str(
            "(preference pref-name (over all (= x y)))"
        )
        .is_value(PreferenceTimedGoalDescription::Preference(
            Some("pref-name".into()),
            TimedGoalDescription::over(
                Interval::All,
                GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            )
        )));
    }

    #[test]
    fn doc_example_0() {
        assert!(
            PreferenceTimedGoalDescription::from_str("(at start (= x y))").is_value(
                PreferenceTimedGoalDescription::Required(TimedGoalDescription::at(
                    TimeSpecifier::Start,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        assert!(
            PreferenceTimedGoalDescription::from_str("(preference (over all (= x y)))").is_value(
                PreferenceTimedGoalDescription::Preference(
                    None,
                    TimedGoalDescription::over(
                        Interval::All,
                        GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                )
            )
        );

        assert!(PreferenceTimedGoalDescription::from_str(
            "(preference pref-name (over all (= x y)))"
        )
        .is_value(PreferenceTimedGoalDescription::Preference(
            Some("pref-name".into()),
            TimedGoalDescription::over(
                Interval::All,
                GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            )
        )));
    }
}
