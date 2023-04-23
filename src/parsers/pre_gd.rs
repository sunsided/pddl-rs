//! Provides parsers for goal definitions.

use crate::parsers::{parens, prefix_expr, space_separated_list0, typed_list};
use crate::parsers::{parse_pref_gd, parse_variable};
use crate::types::PreGD;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_pre_gd};
/// # use pddl::types::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Preference, PreferenceName, PreferenceGD, PreGD, Term, Variable};
/// use pddl::types::{Type, Typed, TypedList};
///
/// assert_eq!(parse_pre_gd("(= x y)"), Ok(("",
///     PreGD::Preference(
///         PreferenceGD::Goal(
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
/// assert_eq!(parse_pre_gd("(and (= x y) (= a b))"), Ok(("",
///     PreGD::new_and([
///         PreGD::Preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )),
///         PreGD::Preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("a".into()),
///                     Term::Name("b".into())
///                 )
///             )
///         ))
///     ])
/// )));
///
/// assert_eq!(parse_pre_gd("(forall (?a ?b) (= a b))"), Ok(("",
///     PreGD::new_forall(
///         TypedList::from_iter([
///             Typed::new(Variable::from_str("a"), Type::OBJECT),
///             Typed::new(Variable::from_str("b"), Type::OBJECT),
///         ]),
///         PreGD::Preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("a".into()),
///                     Term::Name("b".into())
///                 )
///             )
///         ))
///     )
/// )));
/// ```
pub fn parse_pre_gd(input: &str) -> IResult<&str, PreGD> {
    let pref_gd = map(parse_pref_gd, PreGD::new_preference);
    let and = map(
        prefix_expr("and", space_separated_list0(parse_pre_gd)),
        PreGD::new_and,
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_pre_gd),
            )),
        ),
        |(vars, gd)| PreGD::new_forall(vars, gd),
    );

    alt((forall, and, pref_gd))(input)
}
