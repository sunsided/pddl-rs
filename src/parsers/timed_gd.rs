//! Provides parsers for timed goal definitions (`<timed-GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_gd, parse_interval, parse_time_specifier, prefix};
use crate::types::TimedGoalDefinition;

/// Parses a timed goal definition (`<timed-GD>`), i.e. `(at <time-specifier> <GD>) | (over <interval> <GD>)`.
pub(crate) fn parse_timed_gd(node: &Node) -> R<TimedGoalDefinition> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "at", |args| {
                    Ok((parse_time_specifier(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(TimedGoalDefinition::from)
            },
            &|n| {
                prefix(n, "over", |args| {
                    Ok((parse_interval(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(TimedGoalDefinition::from)
            },
        ],
    )
}

crate::parsers::impl_parser!(TimedGoalDefinition => parse_timed_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn it_works() {
        let input = "(over all (can-move ?from-waypoint ?to-waypoint))";
        let _gd = TimedGoalDefinition::from_str(input).unwrap();
    }

    #[test]
    fn test_parse() {
        assert!(
            TimedGoalDefinition::from_str("(at start (= x y))").is_value(TimedGoalDefinition::at(
                TimeSpecifier::Start,
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        assert!(
            TimedGoalDefinition::from_str("(over all (= x y))").is_value(
                TimedGoalDefinition::over(
                    Interval::All,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
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
            TimedGoalDefinition::from_str("(at start (= x y))").is_value(TimedGoalDefinition::at(
                TimeSpecifier::Start,
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        assert!(
            TimedGoalDefinition::from_str("(over all (= x y))").is_value(
                TimedGoalDefinition::over(
                    Interval::All,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            )
        );
    }
}
