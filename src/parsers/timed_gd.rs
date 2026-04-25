//! Provides parsers for timed goal definitions.

use crate::parsers::{parse_gd, parse_interval, parse_time_specifier};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::TimedGoalDefinition;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

/// Parser for timed goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_timed_gd, preamble::*};
/// # use pddl::{AtomicFormula, GoalDefinition, Interval, Term, TimedGoalDefinition, TimeSpecifier};
/// assert!(parse_timed_gd("(at start (= x y))").is_value(
///     TimedGoalDefinition::new_at(
///         TimeSpecifier::Start,
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// ));
///
/// assert!(parse_timed_gd("(over all (= x y))").is_value(
///     TimedGoalDefinition::new_over(
///         Interval::All,
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// ));
/// ```
pub fn parse_timed_gd<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, TimedGoalDefinition> {
    let at = map(
        prefix_expr(
            "at",
            (parse_time_specifier, preceded(multispace1, parse_gd)),
        ),
        TimedGoalDefinition::from,
    );

    let over = map(
        prefix_expr("over", (parse_interval, preceded(multispace1, parse_gd))),
        TimedGoalDefinition::from,
    );

    alt((at, over)).parse(input.into())
}

impl crate::parsers::Parser for TimedGoalDefinition {
    type Item = TimedGoalDefinition;

    /// See [`parse_timed_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_timed_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::UnwrapValue;
    use crate::{AtomicFormula, GoalDefinition, Interval, Parser, Term, TimeSpecifier};

    #[test]
    fn it_works() {
        let input = "(over all (can-move ?from-waypoint ?to-waypoint))";
        let (_, _gd) = parse_timed_gd(Span::new(input)).unwrap();
    }

    #[test]
    fn test_parse() {
        assert!(TimedGoalDefinition::parse("(at start (= x y))").is_value(
            TimedGoalDefinition::new_at(
                TimeSpecifier::Start,
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            )
        ));

        assert!(
            parse_timed_gd("(over all (= x y))").is_value(TimedGoalDefinition::new_over(
                Interval::All,
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );
    }
}
