//! Provides parsers for problem goal descriptions (`<goal>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_pre_gd, prefix};
use crate::types::ProblemGoalDescription;

/// Parses a problem goal description (`<goal>`), i.e. `(:goal <pre-GD>)`.
pub(crate) fn parse_problem_goal_def(node: &Node) -> R<ProblemGoalDescription> {
    prefix(node, ":goal", |args| {
        parse_pre_gd(args.next()?).map(ProblemGoalDescription::new)
    })
}

crate::parsers::impl_parser!(ProblemGoalDescription => parse_problem_goal_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:goal (= x y))";
        assert!(
            ProblemGoalDescription::from_str(input).is_value(ProblemGoalDescription::from(
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
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
            ProblemGoalDescription::from_str(input).is_value(ProblemGoalDescription::from(
                PreconditionGoalDescription::Preference(PreferenceGoalDescription::Goal(
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            ))
        );
    }
}
