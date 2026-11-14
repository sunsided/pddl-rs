//! Provides parsers for problem goal definitions (`<goal>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_pre_gd, prefix};
use crate::types::ProblemGoalDefinition;

/// Parses a problem goal definition (`<goal>`), i.e. `(:goal <pre-GD>)`.
pub(crate) fn parse_problem_goal_def(node: &Node) -> R<ProblemGoalDefinition> {
    prefix(node, ":goal", |args| {
        parse_pre_gd(args.next()?).map(ProblemGoalDefinition::new)
    })
}

crate::parsers::impl_parser!(ProblemGoalDefinition => parse_problem_goal_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:goal (= x y))";
        assert!(
            ProblemGoalDefinition::from_str(input).is_value(ProblemGoalDefinition::from(
                PreconditionGoalDefinition::Preference(PreferenceGoalDefinition::Goal(
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "(:goal (= x y))";
        assert!(
            ProblemGoalDefinition::from_str(input).is_value(ProblemGoalDefinition::from(
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
