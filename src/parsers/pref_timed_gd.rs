//! Provides parsers for (preferred) timed goal definitions.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::terminated;
use nom::Parser;

use crate::parsers::{parse_pref_name, parse_timed_gd};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::PreferenceTimedGoalDefinition;

/// Parser for (preferred) timed goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_pref_timed_gd, preamble::*};
/// # use pddl::{AtomicFormula, GoalDefinition, Interval, PreferenceTimedGoalDefinition, Term, TimedGoalDefinition, TimeSpecifier};
/// assert!(parse_pref_timed_gd("(at start (= x y))").is_value(
///     PreferenceTimedGoalDefinition::Required(
///         TimedGoalDefinition::at(
///             TimeSpecifier::Start,
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )
///     )
/// ));
///
///
/// assert!(parse_pref_timed_gd("(preference (over all (= x y)))").is_value(
///     PreferenceTimedGoalDefinition::Preference(
///         None,
///         TimedGoalDefinition::over(
///             Interval::All,
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )
///     )
/// ));
///
/// assert!(parse_pref_timed_gd("(preference pref-name (over all (= x y)))").is_value(
///     PreferenceTimedGoalDefinition::Preference(
///         Some("pref-name".into()),
///         TimedGoalDefinition::over(
///             Interval::All,
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )
///     )
/// ));
/// ```
pub fn parse_pref_timed_gd<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, PreferenceTimedGoalDefinition> {
    let required = map(parse_timed_gd, PreferenceTimedGoalDefinition::from);

    // :preferences
    let preference = map(
        prefix_expr(
            "preference",
            (
                opt(terminated(parse_pref_name, multispace1)),
                parse_timed_gd,
            ),
        ),
        PreferenceTimedGoalDefinition::from,
    );

    alt((preference, required)).parse(input.into())
}

impl crate::parsers::Parser for PreferenceTimedGoalDefinition {
    type Item = PreferenceTimedGoalDefinition;

    /// See [`parse_pref_timed_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pref_timed_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{
        AtomicFormula, GoalDefinition, Interval, PreferenceTimedGoalDefinition, Term,
        TimeSpecifier, TimedGoalDefinition,
    };

    #[test]
    fn test_parse() {
        assert!(
            PreferenceTimedGoalDefinition::parse("(at start (= x y))").is_value(
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
            PreferenceTimedGoalDefinition::parse("(preference (over all (= x y)))").is_value(
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

        assert!(
            PreferenceTimedGoalDefinition::parse("(preference pref-name (over all (= x y)))")
                .is_value(PreferenceTimedGoalDefinition::Preference(
                    Some("pref-name".into()),
                    TimedGoalDefinition::over(
                        Interval::All,
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                ))
        );
    }
}
