//! Provides parsers for (preferred) timed goal definitions.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{terminated, tuple};

use crate::parsers::{parse_pref_name, parse_timed_gd};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::PrefTimedGD;

/// Parser for (preferred) timed goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_pref_timed_gd, preamble::*};
/// # use pddl::{AtomicFormula, GoalDefinition, Interval, PrefTimedGD, Term, TimedGD, TimeSpecifier};
/// assert!(parse_pref_timed_gd("(at start (= x y))").is_value(
///     PrefTimedGD::Required(
///         TimedGD::new_at(
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
///     PrefTimedGD::Preference(
///         None,
///         TimedGD::new_over(
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
///     PrefTimedGD::Preference(
///         Some("pref-name".into()),
///         TimedGD::new_over(
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
pub fn parse_pref_timed_gd<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, PrefTimedGD> {
    let required = map(parse_timed_gd, PrefTimedGD::from);

    // :preferences
    let preference = map(
        prefix_expr(
            "preference",
            tuple((
                opt(terminated(parse_pref_name, multispace1)),
                parse_timed_gd,
            )),
        ),
        PrefTimedGD::from,
    );

    alt((preference, required))(input.into())
}

impl crate::parsers::Parser for PrefTimedGD {
    type Item = PrefTimedGD;

    /// See [`parse_pref_timed_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pref_timed_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{
        AtomicFormula, GoalDefinition, Interval, PrefTimedGD, Term, TimeSpecifier, TimedGD,
    };

    #[test]
    fn test_parse() {
        assert!(
            PrefTimedGD::parse("(at start (= x y))").is_value(PrefTimedGD::Required(
                TimedGD::new_at(
                    TimeSpecifier::Start,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )
            ))
        );

        assert!(
            PrefTimedGD::parse("(preference (over all (= x y)))").is_value(
                PrefTimedGD::Preference(
                    None,
                    TimedGD::new_over(
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
            PrefTimedGD::parse("(preference pref-name (over all (= x y)))").is_value(
                PrefTimedGD::Preference(
                    Some("pref-name".into()),
                    TimedGD::new_over(
                        Interval::All,
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )
                )
            )
        );
    }
}
