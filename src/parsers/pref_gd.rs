//! Provides parsers for preference goal definitions.

use nom::branch::alt;
use nom::combinator::{map, opt};
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::sep;
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
    // :preferences; the name is optional, so this also covers `(preference <GD>)`.
    let pref_named = map(
        prefix_expr(
            "preference",
            (opt(parse_pref_name), preceded(sep, parse_gd)),
        ),
        |(pref, gd)| PreferenceGoalDefinition::from_preference(Preference::new(pref, gd)),
    );

    let gd = map(parse_gd, PreferenceGoalDefinition::from_gd);

    alt((pref_named, gd)).parse(input.into())
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
