//! Provides parsers for domain structure definitions.

use nom::branch::alt;
use nom::combinator::map;

use crate::parsers::{parse_action_def, parse_da_def, parse_derived_predicate, ParseResult, Span};
use crate::types::StructureDef;

/// Parses a domain structure definition.
///
/// ## Example
///
/// ```
/// # use pddl::parsers::{parse_structure_def, preamble::*};
/// # use pddl::{ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effects, GoalDefinition, Literal, PEffect, Predicate, Preference, PreferenceGD, PreconditionGoalDefinitions, StructureDef, Term, Variable};
/// # use pddl::{Name, ToTyped, TypedList};
/// let input = r#"(:action take-out
///                     :parameters (?x - physob)
///                     :precondition (not (= ?x B))
///                     :effect (not (in ?x))
///                 )"#;
///
/// let action = parse_structure_def(input);
///
/// assert!(action.is_value(
///     StructureDef::new_action(ActionDefinition::new(
///         ActionSymbol::from("take-out"),
///         TypedList::from_iter([
///             Variable::from("x").to_typed("physob")
///         ]),
///         PreconditionGoalDefinitions::new_preference(PreferenceGD::from_gd(
///             GoalDefinition::new_not(
///                 GoalDefinition::new_atomic_formula(
///                     AtomicFormula::new_equality(
///                         Term::Variable(Variable::from("x")),
///                         Term::Name(Name::new("B"))
///                     )
///                 )
///             )
///         )),
///         Some(Effects::new(CEffect::new_p_effect(
///             PEffect::NotAtomicFormula(
///                 AtomicFormula::new_predicate(
///                     Predicate::from("in"),
///                     vec![Term::Variable(Variable::from("x"))]
///                 )
///             )
///         )))
///     )
/// )));
/// ```
pub fn parse_structure_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, StructureDef> {
    let action = map(parse_action_def, StructureDef::new_action);
    // :durative-actions
    let durative = map(parse_da_def, StructureDef::new_durative_action);
    // :derived-predicates
    let derived = map(parse_derived_predicate, StructureDef::new_derived);
    alt((derived, action, durative))(input.into())
}

impl crate::parsers::Parser for StructureDef {
    type Item = StructureDef;

    /// See [`parse_structure_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_structure_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{
        ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effects, GoalDefinition, Name,
        PEffect, PreconditionGoalDefinitions, Predicate, PreferenceGD, StructureDef, Term, ToTyped,
        TypedList, Variable,
    };

    #[test]
    fn test_parse() {
        let input = r#"(:action take-out
                             :parameters (?x - physob)
                             :precondition (not (= ?x B))
                             :effect (not (in ?x))
                         )"#;

        let action = StructureDef::parse(input);

        assert!(
            action.is_value(StructureDef::new_action(ActionDefinition::new(
                ActionSymbol::from("take-out"),
                TypedList::from_iter([Variable::from("x").to_typed("physob")]),
                PreconditionGoalDefinitions::new_preference(PreferenceGD::from_gd(
                    GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
                        AtomicFormula::new_equality(
                            Term::Variable(Variable::from("x")),
                            Term::Name(Name::new("B"))
                        )
                    ))
                )),
                Some(Effects::new(CEffect::new_p_effect(
                    PEffect::NotAtomicFormula(AtomicFormula::new_predicate(
                        Predicate::from("in"),
                        vec![Term::Variable(Variable::from("x"))]
                    ))
                )))
            )))
        );
    }
}
