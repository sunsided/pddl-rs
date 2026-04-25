//! Provides parsers for preference goal definitions.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::{parse_gd, parse_pref_name};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::{Preference, PreferenceGoalDefinition};

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_pref_gd, preamble::*};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Preference, PreferenceName, PreferenceGoalDefinition, Term, Variable};
/// // Simple goal definition.
/// assert!(parse_pref_gd("(= x y)").is_value(
///     PreferenceGoalDefinition::Goal(
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// ));
///
/// // Named preference.
/// assert!(parse_pref_gd("(preference p (= x y))").is_value(
///     PreferenceGoalDefinition::Preference(
///         Preference::new(
///             Some(PreferenceName::from("p")),
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
/// // Unnamed preference.
/// assert!(parse_pref_gd("(preference (= x y))").is_value(
///     PreferenceGoalDefinition::Preference(
///         Preference::new(
///             None,
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
pub fn parse_pref_gd<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, PreferenceGoalDefinition> {
    // :preferences
    let pref_named = map(
        prefix_expr(
            "preference",
            (opt(parse_pref_name), preceded(multispace1, parse_gd)),
        ),
        |(pref, gd)| PreferenceGoalDefinition::from_preference(Preference::new(pref, gd)),
    );

    let pref_unnamed = map(prefix_expr("preference", parse_gd), |gd| {
        PreferenceGoalDefinition::from_preference(Preference::new(None, gd))
    });

    let gd = map(parse_gd, PreferenceGoalDefinition::from_gd);

    alt((pref_named, pref_unnamed, gd)).parse(input.into())
}

impl crate::parsers::Parser for PreferenceGoalDefinition {
    type Item = PreferenceGoalDefinition;

    /// See [`parse_pref_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pref_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{
        AtomicFormula, GoalDefinition, Preference, PreferenceGoalDefinition, PreferenceName, Term,
    };

    #[test]
    fn test_parse() {
        // Simple goal definition.
        assert!(PreferenceGoalDefinition::parse("(= x y)").is_value(
            PreferenceGoalDefinition::Goal(GoalDefinition::AtomicFormula(
                AtomicFormula::new_equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        ));

        // Named preference.
        assert!(
            PreferenceGoalDefinition::parse("(preference p (= x y))").is_value(
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
            PreferenceGoalDefinition::parse("(preference (= x y))").is_value(
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
