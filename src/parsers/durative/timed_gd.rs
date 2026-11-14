//! Provides parsers for timed goal descriptions (`<timed-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_gd, parse_interval, parse_time_specifier, prefix};
use crate::types::TimedGoalDescription;

/// Parses a timed goal description (`<timed-GD>`), i.e. `(at <time-specifier> <GD>) | (over <interval> <GD>)`.
pub(crate) fn parse_timed_gd(node: &Node) -> R<TimedGoalDescription> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "at", |args| {
                    Ok((parse_time_specifier(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(TimedGoalDescription::from)
            },
            &|n| {
                prefix(n, "over", |args| {
                    Ok((parse_interval(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(TimedGoalDescription::from)
            },
        ],
    )
}

crate::parsers::impl_parser!(TimedGoalDescription => parse_timed_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn it_works() {
        let input = "(over all (can-move ?from-waypoint ?to-waypoint))";
        let _gd = TimedGoalDescription::from_str(input).unwrap();
    }

    #[test]
    fn test_parse() {
        assert!(
            TimedGoalDescription::from_str("(at start (= x y))").is_value(
                TimedGoalDescription::at(
                    TimeSpecifier::Start,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            )
        );

        assert!(
            TimedGoalDescription::from_str("(over all (= x y))").is_value(
                TimedGoalDescription::over(
                    Interval::All,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            )
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            TimedGoalDescription::from_str("(at start (= x y))").is_value(
                TimedGoalDescription::at(
                    TimeSpecifier::Start,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            )
        );

        assert!(
            TimedGoalDescription::from_str("(over all (= x y))").is_value(
                TimedGoalDescription::over(
                    Interval::All,
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            )
        );
    }
}
