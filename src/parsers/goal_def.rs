//! Provides parsers for pre-GD goal definitions.

use crate::parsers::{parse_pre_gd, prefix_expr, ParseResult, Span};
use crate::types::GoalDef;
use nom::combinator::map;

/// Parses pre-GD goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_goal_def, preamble::*};
/// # use pddl::{AtomicFormula, GoalDef, GoalDefinition, PreferenceGD, PreconditionGoalDefinition, Term};
/// let input = "(:goal (= x y))";
/// assert!(parse_problem_goal_def(input).is_value(
///     GoalDef::from(
///         PreconditionGoalDefinition::Preference(
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
/// ));
/// ```
pub fn parse_problem_goal_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, GoalDef> {
    map(prefix_expr(":goal", parse_pre_gd), GoalDef::new)(input.into())
}

impl crate::parsers::Parser for GoalDef {
    type Item = GoalDef;

    /// See [`parse_problem_goal_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_goal_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{
        AtomicFormula, GoalDef, GoalDefinition, Parser, PreconditionGoalDefinition, PreferenceGD,
        Term,
    };

    #[test]
    fn test_parse() {
        let input = "(:goal (= x y))";
        assert!(GoalDef::parse(input).is_value(GoalDef::from(
            PreconditionGoalDefinition::Preference(PreferenceGD::Goal(
                GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        )));
    }
}
