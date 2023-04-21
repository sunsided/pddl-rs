//! Provides parsers for durative action goal definitions.

use crate::parsers::{
    parens, parse_pref_timed_gd, parse_variable, prefix_expr, space_separated_list0, typed_list,
};
use crate::types::DAGD;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_da_gd};
/// # use pddl::types::{AtomicFormula, EqualityAtomicFormula, GD, Literal, Name, Preference, PreferenceName, PreferenceGD, PreGD, Term, Type, Typed, TypedList, Variable, DAGD, PrefTimedGD, TimedGD, TimeSpecifier, Interval};
///
/// assert_eq!(parse_da_gd("(at start (= x y))"), Ok(("",
///     DAGD::Timed(
///         PrefTimedGD::Required(
///             TimedGD::new_at(
///                 TimeSpecifier::Start,
///                 GD::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("x".into()),
///                         Term::Name("y".into())
///                     )
///                 )
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_da_gd("(and )"), Ok(("",
///     DAGD::new_and([])
/// )));
///
/// assert_eq!(parse_da_gd("(and (at start (= x y)) (over all (= a b)))"), Ok(("",
///     DAGD::new_and([
///         DAGD::Timed(PrefTimedGD::Required(
///             TimedGD::new_at(
///                 TimeSpecifier::Start,
///                 GD::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("x".into()),
///                         Term::Name("y".into())
///                     )
///                 )
///             )
///         )),
///         DAGD::Timed(PrefTimedGD::Required(
///             TimedGD::new_over(
///                 Interval::All,
///                 GD::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("a".into()),
///                         Term::Name("b".into())
///                     )
///                 )
///             )
///         ))
///     ])
/// )));
///
/// assert_eq!(parse_da_gd("(forall (?a ?b) (at start (= a b)))"), Ok(("",
///     DAGD::new_forall(
///         TypedList::from_iter([
///             Typed::new_object(Variable::from_str("a")),
///             Typed::new_object(Variable::from_str("b")),
///         ]),
///         DAGD::Timed(
///             PrefTimedGD::Required(
///                 TimedGD::new_at(
///                     TimeSpecifier::Start,
///                     GD::AtomicFormula(
///                         AtomicFormula::new_equality(
///                             Term::Name("a".into()),
///                             Term::Name("b".into())
///                         )
///                     )
///                 )
///             )
///         )
///     )
/// )));
/// ```
pub fn parse_da_gd(input: &str) -> IResult<&str, DAGD> {
    let pref_timed_gd = map(parse_pref_timed_gd, DAGD::new_timed);
    let and = map(
        prefix_expr("and", space_separated_list0(parse_da_gd)),
        DAGD::new_and,
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_da_gd),
            )),
        ),
        |(vars, gd)| DAGD::new_forall(vars, gd),
    );

    alt((forall, and, pref_timed_gd))(input)
}
