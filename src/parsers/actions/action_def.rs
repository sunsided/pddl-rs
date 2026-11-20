//! Provides parsers for action definitions (`<action-def>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    empty_or, keyword_arg, opt_keyword_arg, parse_action_symbol, parse_da_effect, parse_effect,
    parse_pre_gd, prefix, typed_variables,
};
use crate::types::{ActionDefinition, ProcessDefinition};

/// Parses an action definition (`<action-def>`), i.e. `(:action <action-symbol> :parameters (<typed list (variable)>)
/// [:precondition <pre-GD>] [:effect <effect>])`. `()` stands for no precondition or effect.
pub(crate) fn parse_action_def(node: &Node) -> R<ActionDefinition> {
    parse_action_like(node, ":action")
}

/// Parses a PDDL+ event definition, i.e. `(:event <name> :parameters (<typed list (variable)>)
/// [:precondition <pre-GD>] [:effect <effect>])`, which has the form of an action.
pub(crate) fn parse_event_def(node: &Node) -> R<ActionDefinition> {
    parse_action_like(node, ":event")
}

/// Parses a PDDL+ process definition, i.e. `(:process <name> :parameters (<typed list (variable)>)
/// [:precondition <pre-GD>] [:effect <da-effect>])`. The effect is read like a durative
/// action's, so its continuous effects keep `#t`.
pub(crate) fn parse_process_def(node: &Node) -> R<ProcessDefinition> {
    prefix(node, ":process", |args| {
        let symbol = parse_action_symbol(args.next()?)?;
        let parameters = keyword_arg(args, ":parameters", typed_variables)?;
        let precondition = opt_keyword_arg(args, ":precondition", |n| empty_or(n, parse_pre_gd))?;
        let effect = opt_keyword_arg(args, ":effect", |n| empty_or(n, parse_da_effect))?;
        args.reject_misspelled(&[":precondition", ":effect"])?;
        Ok(ProcessDefinition::new(
            symbol,
            parameters,
            precondition.flatten().into(),
            effect.flatten(),
        ))
    })
}

/// Parses `(<keyword> <name> :parameters ... [:precondition ...] [:effect ...])`, the
/// form actions and events share.
fn parse_action_like(node: &Node, keyword: &'static str) -> R<ActionDefinition> {
    prefix(node, keyword, |args| {
        let symbol = parse_action_symbol(args.next()?)?;
        let parameters = keyword_arg(args, ":parameters", typed_variables)?;
        let precondition = opt_keyword_arg(args, ":precondition", |n| empty_or(n, parse_pre_gd))?;
        let effect = opt_keyword_arg(args, ":effect", |n| empty_or(n, parse_effect))?;
        args.reject_misspelled(&[":precondition", ":effect"])?;
        Ok(ActionDefinition::new(
            symbol,
            parameters,
            precondition.flatten().into(),
            effect.flatten(),
        ))
    })
}

crate::parsers::impl_parser!(ActionDefinition => parse_action_def);
crate::parsers::impl_parser!(ProcessDefinition => parse_process_def);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = r"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )";

        let action = ActionDefinition::from_str(input).unwrap();

        check!(
            action
                == ActionDefinition::new(
                    ActionSymbol::string("take-out"),
                    TypedList::from_iter([Variable::new_string("x").to_typed("physob")]),
                    PreconditionGoalDescriptions::from_str("(not (= ?x B))").unwrap(),
                    Some(Effects::from_str("(not (in ?x))").unwrap())
                )
        );
    }

    #[test]
    fn doc_example_0() {
        let input = r"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )";

        let action = ActionDefinition::from_str(input);

        check!(action.is_value(ActionDefinition::new(
            ActionSymbol::string("take-out"),
            TypedList::from_iter([Variable::new_string("x").to_typed("physob")]),
            PreconditionGoalDescriptions::from(PreconditionGoalDescription::Preference(
                PreferenceGoalDescription::from_gd(GoalDescription::new_not(
                    GoalDescription::AtomicFormula(AtomicFormula::new_equality(
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
        let input = r"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )";

        let action = ActionDefinition::from_str(input).unwrap();

        check!(
            action
                == ActionDefinition::new(
                    ActionSymbol::string("take-out"),
                    TypedList::from_iter([Variable::new_string("x").to_typed("physob")]),
                    PreconditionGoalDescriptions::from_str("(not (= ?x B))").unwrap(),
                    Some(Effects::from_str("(not (in ?x))").unwrap())
                )
        );
    }

    #[test]
    fn parses_pddl_plus_processes_and_events() {
        let process = ProcessDefinition::from_str(
            "(:process fall :parameters (?b - ball)
               :precondition (falling ?b)
               :effect (and (increase (height ?b) (* #t (velocity ?b)))
                            (increase (velocity ?b) (* #t 9.8))))",
        )
        .unwrap();
        check!(process.symbol() == &ActionSymbol::string("fall"));
        check!(process.parameters().len() == 1);
        check!(process.precondition().len() == 1);
        let Some(DurativeActionEffect::All(effects)) = process.effect() else {
            panic!("{process:?}");
        };
        check!(effects.len() == 2);
        check!(let DurativeActionEffect::Timed(TimedEffect::ContinuousEffect(..)) = &effects[0]);

        let event = StructureDef::from_str(
            "(:event bounce :parameters (?b - ball)
               :precondition (and (falling ?b) (<= (height ?b) 0))
               :effect (assign (velocity ?b) (* 0.8 (velocity ?b))))",
        );
        check!(let Ok(StructureDef::Event(_)) = event);

        let without_effect = StructureDef::from_str("(:process idle :parameters ())").unwrap();
        let StructureDef::Process(process) = without_effect else {
            panic!("{without_effect:?}");
        };
        check!(process.effect().is_none());
        check!(process.precondition().is_empty());

        check!(ProcessDefinition::from_str("(:action a :parameters ())").is_err());
    }
}
