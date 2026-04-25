//! Provides parsers for pre-GD goal definitions.

use crate::parsers::{parse_pre_gd, prefix_expr, ParseResult, Span};
use crate::types::ProblemGoalDefinition;
use nom::combinator::map;
use nom::Parser;

/// Parses pre-GD goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_goal_def, preamble::*};
/// # use pddl::{AtomicFormula, GoalDefinition, ProblemGoalDefinition, PreferenceGoalDefinition, PreconditionGoalDefinition, Term};
/// let input = "(:goal (= x y))";
/// assert!(parse_problem_goal_def(input).is_value(
///     ProblemGoalDefinition::from(
///         PreconditionGoalDefinition::Preference(
///             PreferenceGoalDefinition::Goal(
///                 GoalDefinition::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("x".into()),
///                         Term::Name("y".into())
///                     )
///                 )
///             )
///         )
///     )
/// ));
/// ```
pub fn parse_problem_goal_def<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, ProblemGoalDefinition> {
    map(
        prefix_expr(":goal", parse_pre_gd),
        ProblemGoalDefinition::new,
    )
    .parse(input.into())
}

impl crate::parsers::Parser for ProblemGoalDefinition {
    type Item = ProblemGoalDefinition;

    /// See [`parse_problem_goal_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_goal_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{
        AtomicFormula, GoalDefinition, Parser, PreconditionGoalDefinition,
        PreferenceGoalDefinition, ProblemGoalDefinition, Term,
    };

    #[test]
    fn test_parse() {
        let input = "(:goal (= x y))";
        assert!(
            ProblemGoalDefinition::parse(input).is_value(ProblemGoalDefinition::from(
                PreconditionGoalDefinition::Preference(PreferenceGoalDefinition::Goal(
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            ))
        );
    }
}
