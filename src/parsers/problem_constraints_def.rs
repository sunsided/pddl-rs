//! Provides parsers for problem constraint definitions.

use crate::parsers::{parse_pref_con_gd, prefix_expr, ParseResult, Span};
use crate::types::ProblemConstraintsDef;
use nom::combinator::map;

/// Parser that parses problem constraint definitions, i.e. `(:constraints <pref-con-GD>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_constraints_def, preamble::*};
/// # use pddl::{ConGD, ProblemConstraintsDef, PrefConGDs};
/// let input = "(:constraints (preference test (and)))";
/// assert!(parse_problem_constraints_def(input.into()).is_value(
///     ProblemConstraintsDef::new(
///         PrefConGDs::new_preference(Some("test".into()), ConGD::new_and([]))
///     )
/// ));
/// ```
pub fn parse_problem_constraints_def(input: Span) -> ParseResult<ProblemConstraintsDef> {
    // :constraints
    map(
        prefix_expr(":constraints", parse_pref_con_gd),
        ProblemConstraintsDef::new,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for ProblemConstraintsDef<'a> {
    type Item = ProblemConstraintsDef<'a>;

    /// See [`parse_problem_constraints_def`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_constraints_def(input.into())
    }
}
