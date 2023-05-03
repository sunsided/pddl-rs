//! Provides parsers for goal definitions.

use crate::parsers::{parens, prefix_expr, space_separated_list0, typed_list};
use crate::parsers::{parse_pref_gd, parse_variable};
use crate::types::PreconditionGoalDefinition;
use crate::PreconditionGoalDefinitions;
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
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Preference, PreferenceName, PreferenceGD, PreconditionGoalDefinitions, PreconditionGoalDefinition, Term, Variable, Type, Typed, TypedList};
/// assert_eq!(parse_pre_gd("(= x y)"), Ok(("",
///     PreconditionGoalDefinitions::new_preference(
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
///     PreconditionGoalDefinitions::from_iter([
///         PreconditionGoalDefinition::Preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )),
///         PreconditionGoalDefinition::Preference(PreferenceGD::Goal(
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
///     PreconditionGoalDefinition::new_forall(
///         TypedList::from_iter([
///             Typed::new(Variable::from_str("a"), Type::OBJECT),
///             Typed::new(Variable::from_str("b"), Type::OBJECT),
///         ]),
///         PreconditionGoalDefinition::new_preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("a".into()),
///                     Term::Name("b".into())
///                 )
///             )
///         )).into()
///     ).into()
/// )));
/// ```
pub fn parse_pre_gd(input: &str) -> IResult<&str, PreconditionGoalDefinitions> {
    let pref_gd = map(parse_pref_gd, |x| {
        PreconditionGoalDefinitions::from(PreconditionGoalDefinition::new_preference(x))
    });
    let and = map(
        prefix_expr("and", space_separated_list0(parse_pre_gd)),
        |x| PreconditionGoalDefinitions::from_iter(x.into_iter().flatten()),
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
        |(vars, gd)| {
            PreconditionGoalDefinitions::from(PreconditionGoalDefinition::new_forall(vars, gd))
        },
    );

    alt((forall, and, pref_gd))(input)
}

impl<'a> crate::parsers::Parser<'a> for PreconditionGoalDefinitions<'a> {
    type Item = PreconditionGoalDefinitions<'a>;

    /// See [`parse_pre_gd`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_pre_gd(input)
    }
}
