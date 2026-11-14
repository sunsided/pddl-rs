//! Provides parsers for structure definitions (`<structure-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_action_def, parse_da_def, parse_derived_predicate};
use crate::types::StructureDef;

/// Parses a structure definition (`<structure-def>`), i.e. `<action-def> | <durative-action-def> | <derived-def>`.
pub(crate) fn parse_structure_def(node: &Node) -> R<StructureDef> {
    alt(
        node,
        &[
            // :derived-predicates
            &|n| parse_derived_predicate(n).map(StructureDef::new_derived),
            &|n| parse_action_def(n).map(StructureDef::new_action),
            // :durative-actions
            &|n| parse_da_def(n).map(StructureDef::new_durative_action),
        ],
    )
}

crate::parsers::impl_parser!(StructureDef => parse_structure_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = r#"(:action take-out
                             :parameters (?x - physob)
                             :precondition (not (= ?x B))
                             :effect (not (in ?x))
                         )"#;

        let action = StructureDef::from_str(input);

        check!(action.is_value(StructureDef::action(ActionDefinition::new(
            ActionSymbol::from("take-out"),
            TypedList::from_iter([Variable::from("x").to_typed("physob")]),
            PreconditionGoalDescriptions::preference(PreferenceGoalDescription::from_gd(
                GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                    Term::Variable(Variable::from("x")),
                    Term::Name(Name::new("B"))
                )))
            )),
            Some(Effects::new(ConditionalEffect::new_primitive_effect(
                PrimitiveEffect::NotAtomicFormula(AtomicFormula::predicate(
                    Predicate::from("in"),
                    vec![Term::Variable(Variable::from("x"))]
                ))
            )))
        ))));
    }

    #[test]
    fn doc_example_0() {
        let input = r#"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )"#;

        let action = StructureDef::from_str(input);

        check!(action.is_value(StructureDef::action(ActionDefinition::new(
            ActionSymbol::from("take-out"),
            TypedList::from_iter([Variable::from("x").to_typed("physob")]),
            PreconditionGoalDescriptions::preference(PreferenceGoalDescription::from_gd(
                GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                    Term::Variable(Variable::from("x")),
                    Term::Name(Name::new("B"))
                )))
            )),
            Some(Effects::new(ConditionalEffect::new_primitive_effect(
                PrimitiveEffect::NotAtomicFormula(AtomicFormula::predicate(
                    Predicate::from("in"),
                    vec![Term::Variable(Variable::from("x"))]
                ))
            )))
        ))));
    }
}
