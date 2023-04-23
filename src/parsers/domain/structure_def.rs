//! Provides parsers for domain structure definitions.

use crate::parsers::domain::{parse_action_def, parse_da_def, parse_derived_predicate};
use crate::types::domain::StructureDef;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parses a domain structure definition.
///
/// ## Example
///
/// ```
/// # use pddl::parsers::domain::{parse_structure_def};
/// # use pddl::types::domain::{ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effect, GoalDefinition, Literal, PEffect, Predicate, Preference, PreferenceGD, PreGD, StructureDef, Term, Variable};
/// # use pddl::types::utility::{Name, ToTyped, TypedList};
///
/// let input = r#"(:action take-out
///                     :parameters (?x - physob)
///                     :precondition (not (= ?x B))
///                     :effect (not (in ?x))
///                 )"#;
///
/// let action = parse_structure_def(input);
///
/// assert_eq!(action, Ok(("",
///     StructureDef::new_action(ActionDefinition::new(
///         ActionSymbol::from("take-out"),
///         TypedList::from_iter([
///             Variable::from("x").to_typed("physob")
///         ]),
///         Some(PreGD::Preference(PreferenceGD::from_gd(
///             GoalDefinition::new_not(
///                 GoalDefinition::new_atomic_formula(
///                     AtomicFormula::new_equality(
///                         Term::Variable(Variable::from("x")),
///                         Term::Name(Name::new("B"))
///                     )
///                 )
///             )
///         ))),
///         Some(Effect::new(CEffect::new_p_effect(
///             PEffect::NotAtomicFormula(
///                 AtomicFormula::new_predicate(
///                     Predicate::from("in"),
///                     vec![Term::Variable(Variable::from("x"))]
///                 )
///             )
///         )))
///     )
/// ))));
/// ```
pub fn parse_structure_def(input: &str) -> IResult<&str, StructureDef> {
    let action = map(parse_action_def, StructureDef::new_action);
    // :durative-actions
    let durative = map(parse_da_def, StructureDef::new_durative_action);
    // :derived-predicates
    let derived = map(parse_derived_predicate, StructureDef::new_derived);
    alt((derived, action, durative))(input)
}
