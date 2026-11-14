//! Provides parsers for problem constraint definitions (`<problem-constraints>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_pref_con_gd, prefix};
use crate::types::ProblemConstraintsDef;

/// Parses a problem constraint definition (`<problem-constraints>`), i.e. `(:constraints <pref-con-GD>)`.
pub(crate) fn parse_problem_constraints_def(node: &Node) -> R<ProblemConstraintsDef> {
    prefix(node, ":constraints", |args| {
        parse_pref_con_gd(args.next()?).map(ProblemConstraintsDef::new)
    })
}

crate::parsers::impl_parser!(ProblemConstraintsDef => parse_problem_constraints_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:constraints (preference test (and)))";
        check!(
            ProblemConstraintsDef::from_str(input).is_value(ProblemConstraintsDef::new(
                PreferenceConstraintGoalDescriptions::preference(
                    Some("test".into()),
                    ConstraintGoalDescription::and([])
                )
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "(:constraints (preference test (and)))";
        check!(
            ProblemConstraintsDef::from_str(input).is_value(ProblemConstraintsDef::new(
                PreferenceConstraintGoalDescriptions::preference(
                    Some("test".into()),
                    ConstraintGoalDescription::and([])
                )
            ))
        );
    }
}
