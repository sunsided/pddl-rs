//! Provides parsers for preference timed goal definitions (`<pref-timed-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_pref_name, parse_timed_gd, prefix};
use crate::types::PreferenceTimedGoalDefinition;

/// Parses a preference timed goal definition (`<pref-timed-GD>`), i.e. `<timed-GD>
/// | (preference [<pref-name>] <timed-GD>)`.
pub(crate) fn parse_pref_timed_gd(node: &Node) -> R<PreferenceTimedGoalDefinition> {
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
                .map(PreferenceTimedGoalDefinition::from)
            },
            &|n| parse_timed_gd(n).map(PreferenceTimedGoalDefinition::from),
        ],
    )
}

crate::parsers::impl_parser!(PreferenceTimedGoalDefinition => parse_pref_timed_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            PreferenceTimedGoalDefinition::from_str("(at start (= x y))").is_value(
                PreferenceTimedGoalDefinition::Required(TimedGoalDefinition::at(
                    TimeSpecifier::Start,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        assert!(
            PreferenceTimedGoalDefinition::from_str("(preference (over all (= x y)))").is_value(
                PreferenceTimedGoalDefinition::Preference(
                    None,
                    TimedGoalDefinition::over(
                        Interval::All,
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                )
            )
        );

        assert!(PreferenceTimedGoalDefinition::from_str(
            "(preference pref-name (over all (= x y)))"
        )
        .is_value(PreferenceTimedGoalDefinition::Preference(
            Some("pref-name".into()),
            TimedGoalDefinition::over(
                Interval::All,
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            )
        )));
    }

    #[test]
    fn doc_example_0() {
        assert!(
            PreferenceTimedGoalDefinition::from_str("(at start (= x y))").is_value(
                PreferenceTimedGoalDefinition::Required(TimedGoalDefinition::at(
                    TimeSpecifier::Start,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        assert!(
            PreferenceTimedGoalDefinition::from_str("(preference (over all (= x y)))").is_value(
                PreferenceTimedGoalDefinition::Preference(
                    None,
                    TimedGoalDefinition::over(
                        Interval::All,
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                )
            )
        );

        assert!(PreferenceTimedGoalDefinition::from_str(
            "(preference pref-name (over all (= x y)))"
        )
        .is_value(PreferenceTimedGoalDefinition::Preference(
            Some("pref-name".into()),
            TimedGoalDefinition::over(
                Interval::All,
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            )
        )));
    }
}
