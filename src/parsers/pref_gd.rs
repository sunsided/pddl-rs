//! Provides parsers for preference goal definitions.

use crate::parsers::{parse_gd, parse_pref_name};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::{Preference, PreferenceGD};
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_pref_gd, preamble::*};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Preference, PreferenceName, PreferenceGD, Term, Variable};
/// // Simple goal definition.
/// assert!(parse_pref_gd("(= x y)").is_value(
///     PreferenceGD::Goal(
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
///     PreferenceGD::Preference(
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
///     PreferenceGD::Preference(
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
pub fn parse_pref_gd<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, PreferenceGD> {
    // :preferences
    let pref_named = map(
        prefix_expr(
            "preference",
            tuple((opt(parse_pref_name), preceded(multispace1, parse_gd))),
        ),
        |(pref, gd)| PreferenceGD::from_preference(Preference::new(pref, gd)),
    );

    let pref_unnamed = map(prefix_expr("preference", parse_gd), |gd| {
        PreferenceGD::from_preference(Preference::new(None, gd))
    });

    let gd = map(parse_gd, PreferenceGD::from_gd);

    alt((pref_named, pref_unnamed, gd))(input.into())
}

impl crate::parsers::Parser for PreferenceGD {
    type Item = PreferenceGD;

    /// See [`parse_pref_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pref_gd(input)
    }
}
