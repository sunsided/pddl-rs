//! Provides parsers for problem constraint definitions.

use crate::parsers::{parse_pref_con_gd, prefix_expr};
use crate::types::PrefConGD;
use nom::IResult;

/// Parser that parses problem constraint definitions, i.e. `(:constraints <pref-con-GD>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_constraints_def};
/// # use pddl::types::{ConGD, PrefConGD};
///
/// let input = "(:constraints (preference test (and)))";
/// assert_eq!(parse_problem_constraints_def(input), Ok(("",
///     PrefConGD::new_preference(Some("test".into()), ConGD::new_and([]))
/// )));
/// ```
pub fn parse_problem_constraints_def(input: &str) -> IResult<&str, PrefConGD> {
    // :constraints
    prefix_expr(":constraints", parse_pref_con_gd)(input)
}
