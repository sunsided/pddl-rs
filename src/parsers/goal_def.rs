//! Provides parsers for pre-GD goal definitions.

use crate::parsers::{parse_pre_gd, prefix_expr};
use crate::types::PreGD;
use nom::IResult;

/// Parses pre-GD goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_goal_def};
/// use pddl::types::{AtomicFormula, GoalDefinition, PreferenceGD, PreGD, Term};
///
/// let input = "(:goal (= x y))";
/// assert_eq!(parse_problem_goal_def(input), Ok(("",
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
/// ```
pub fn parse_problem_goal_def(input: &str) -> IResult<&str, PreGD> {
    prefix_expr(":goal", parse_pre_gd)(input)
}
