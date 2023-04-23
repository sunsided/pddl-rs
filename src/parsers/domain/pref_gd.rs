//! Provides parsers for preference goal definitions.

use crate::parsers::domain::{parse_gd, parse_pref_name};
use crate::parsers::utility::prefix_expr;
use crate::types::domain::{Preference, PreferenceGD};
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::domain::parse_pref_gd;
/// # use pddl::types::domain::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Preference, PreferenceName, PreferenceGD, Term, Variable};
///
/// // Simple goal definition.
/// assert_eq!(parse_pref_gd("(= x y)"), Ok(("",
///     PreferenceGD::GoalDefinition(
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// )));
///
/// // Named preference.
/// assert_eq!(parse_pref_gd("(preference p (= x y))"), Ok(("",
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
/// )));
///
/// // Unnamed preference.
/// assert_eq!(parse_pref_gd("(preference (= x y))"), Ok(("",
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
/// )));
/// ```
pub fn parse_pref_gd(input: &str) -> IResult<&str, PreferenceGD> {
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

    alt((pref_named, pref_unnamed, gd))(input)
}
