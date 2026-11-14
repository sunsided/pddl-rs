//! Provides parsers for domain constraints definitions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{parse_con_gd, prefix};
use crate::types::DomainConstraintsDef;

/// Parses a domain constraints definition, i.e. `(:constraints <con-GD>)`.
pub(crate) fn parse_domain_constraints_def(node: &Node) -> R<DomainConstraintsDef> {
    prefix(node, ":constraints", |args| {
        parse_con_gd(args.next()?).map(DomainConstraintsDef::new)
    })
}

crate::parsers::impl_parser!(DomainConstraintsDef => parse_domain_constraints_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(:constraints (and))";
        assert!(DomainConstraintsDef::from_str(input)
            .is_value(DomainConstraintsDef::new(ConstraintGoalDefinition::and([]))));
    }

    #[test]
    fn doc_example_0() {
        let input = "(:constraints (and))";
        assert!(DomainConstraintsDef::from_str(input)
            .is_value(DomainConstraintsDef::new(ConstraintGoalDefinition::and([]))));
    }
}
