//! Provides parsers for action definitions (`<action-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    empty_or, keyword_arg, opt_keyword_arg, parse_action_symbol, parse_effect, parse_pre_gd,
    prefix, typed_variables,
};
use crate::types::ActionDefinition;

/// Parses an action definition (`<action-def>`), i.e. `(:action <action-symbol> :parameters (<typed list (variable)>)
/// [:precondition <pre-GD>] [:effect <effect>])`. `()` stands for no precondition or effect.
pub(crate) fn parse_action_def(node: &Node) -> R<ActionDefinition> {
    prefix(node, ":action", |args| {
        let symbol = parse_action_symbol(args.next()?)?;
        let parameters = keyword_arg(args, ":parameters", typed_variables)?;
        let precondition = opt_keyword_arg(args, ":precondition", |n| empty_or(n, parse_pre_gd))?;
        let effect = opt_keyword_arg(args, ":effect", |n| empty_or(n, parse_effect))?;
        Ok(ActionDefinition::new(
            symbol,
            parameters,
            precondition.flatten().into(),
            effect.flatten(),
        ))
    })
}

crate::parsers::impl_parser!(ActionDefinition => parse_action_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = r#"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )"#;

        let action = ActionDefinition::from_str(input).unwrap();

        assert_eq!(
            action,
            ActionDefinition::new(
                ActionSymbol::string("take-out"),
                TypedList::from_iter([Variable::new_string("x").to_typed("physob")]),
                PreconditionGoalDefinitions::from_str("(not (= ?x B))").unwrap(),
                Some(Effects::from_str("(not (in ?x))").unwrap())
            )
        );
    }

    #[test]
    fn doc_example_0() {
        let input = r#"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )"#;

        let action = ActionDefinition::from_str(input);

        assert!(action.is_value(ActionDefinition::new(
            ActionSymbol::string("take-out"),
            TypedList::from_iter([Variable::new_string("x").to_typed("physob")]),
            PreconditionGoalDefinitions::from(PreconditionGoalDefinition::Preference(
                PreferenceGoalDefinition::from_gd(GoalDefinition::new_not(
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Variable(Variable::new_string("x")),
                        Term::Name(Name::new("B"))
                    ))
                ))
            )),
            Some(Effects::new(ConditionalEffect::new_primitive_effect(
                PrimitiveEffect::NotAtomicFormula(AtomicFormula::new_predicate(
                    Predicate::new_string("in"),
                    vec![Term::Variable(Variable::new_string("x"))]
                ))
            )))
        )));
    }

    #[test]
    fn doc_example_1() {
        let input = r#"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )"#;

        let action = ActionDefinition::from_str(input).unwrap();

        assert_eq!(
            action,
            ActionDefinition::new(
                ActionSymbol::string("take-out"),
                TypedList::from_iter([Variable::new_string("x").to_typed("physob")]),
                PreconditionGoalDefinitions::from_str("(not (= ?x B))").unwrap(),
                Some(Effects::from_str("(not (in ?x))").unwrap())
            )
        );
    }
}
