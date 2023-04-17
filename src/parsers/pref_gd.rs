//! Provides parsers for preference goal definitions.

use crate::parsers::{parse_gd, parse_pref_name, prefix_expr};
use crate::types::{PrefGD, Preference};
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::parse_pref_gd;
/// # use pddl::types::{AtomicFormula, EqualityAtomicFormula, GD, Literal, Name, Preference, PreferenceName, PrefGD, Term, TypedList, Variable};
///
/// // Simple goal definition.
/// assert_eq!(parse_pref_gd("(= x y)"), Ok(("",
///     PrefGD::GoalDefinition(
///         GD::AtomicFormula(
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
///     PrefGD::Preference(
///         Preference::new(
///             Some(PreferenceName::from("p")),
///             GD::AtomicFormula(
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
///     PrefGD::Preference(
///         Preference::new(
///             None,
///             GD::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )
///     )
/// )));
/// ```
pub fn parse_pref_gd(input: &str) -> IResult<&str, PrefGD> {
    let pref_named = map(
        prefix_expr(
            "preference",
            tuple((opt(parse_pref_name), preceded(multispace1, parse_gd))),
        ),
        |(pref, gd)| PrefGD::from_preference(Preference::new(pref, gd)),
    );

    let pref_unnamed = map(prefix_expr("preference", parse_gd), |gd| {
        PrefGD::from_preference(Preference::new(None, gd))
    });

    let gd = map(parse_gd, PrefGD::from_gd);

    alt((pref_named, pref_unnamed, gd))(input)
}
