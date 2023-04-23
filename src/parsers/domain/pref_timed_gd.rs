//! Provides parsers for (preferred) timed goal definitions.

use crate::parsers::domain::{parse_pref_name, parse_timed_gd};
use crate::parsers::utility::prefix_expr;
use crate::types::domain::PrefTimedGD;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{terminated, tuple};
use nom::IResult;

/// Parser for (preferred) timed goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::domain::{parse_pref_timed_gd};
/// # use pddl::types::domain::{AtomicFormula, GoalDefinition, Interval, PrefTimedGD, Term, TimedGD, TimeSpecifier};
///
/// assert_eq!(parse_pref_timed_gd("(at start (= x y))"), Ok(("",
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
/// )));
///
///
/// assert_eq!(parse_pref_timed_gd("(preference (over all (= x y)))"), Ok(("",
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
/// )));
///
/// assert_eq!(parse_pref_timed_gd("(preference pref-name (over all (= x y)))"), Ok(("",
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
/// )));
/// ```
pub fn parse_pref_timed_gd(input: &str) -> IResult<&str, PrefTimedGD> {
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

    alt((preference, required))(input)
}
