//! Provides parsers for problem constraint definitions.

use crate::parsers::{parse_pref_con_gd, prefix_expr};
use crate::types::ProblemConstraintsDef;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses problem constraint definitions, i.e. `(:constraints <pref-con-GD>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_constraints_def};
/// # use pddl::{ConGD, ProblemConstraintsDef, PrefConGD};
/// let input = "(:constraints (preference test (and)))";
/// assert_eq!(parse_problem_constraints_def(input), Ok(("",
///     ProblemConstraintsDef::new(
///         PrefConGD::new_preference(Some("test".into()), ConGD::new_and([]))
///     )
/// )));
/// ```
pub fn parse_problem_constraints_def(input: &str) -> IResult<&str, ProblemConstraintsDef> {
    // :constraints
    map(
        prefix_expr(":constraints", parse_pref_con_gd),
        ProblemConstraintsDef::new,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for ProblemConstraintsDef<'a> {
    type Item = ProblemConstraintsDef<'a>;

    /// See [`parse_problem_constraints_def`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_problem_constraints_def(input)
    }
}
