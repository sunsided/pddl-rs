//! Provides parsers for pre-GD goal definitions.

use crate::parsers::{parse_pre_gd, prefix_expr};
use crate::types::GoalDef;
use nom::combinator::map;
use nom::IResult;

/// Parses pre-GD goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_goal_def};
/// # use pddl::{AtomicFormula, GoalDef, GoalDefinition, PreferenceGD, PreGD, Term};
/// let input = "(:goal (= x y))";
/// assert_eq!(parse_problem_goal_def(input), Ok(("",
///     GoalDef::new(
///         PreGD::Preference(
///             PreferenceGD::Goal(
///                 GoalDefinition::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("x".into()),
///                         Term::Name("y".into())
///                     )
///                 )
///             )
///         )
///     )
/// )));
/// ```
pub fn parse_problem_goal_def(input: &str) -> IResult<&str, GoalDef> {
    map(prefix_expr(":goal", parse_pre_gd), GoalDef::new)(input)
}

impl<'a> crate::parsers::Parser<'a> for GoalDef<'a> {
    type Item = GoalDef<'a>;

    /// See [`parse_problem_goal_def`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_problem_goal_def(input)
    }
}
