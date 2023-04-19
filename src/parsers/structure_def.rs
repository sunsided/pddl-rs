//! Provides parsers for domain structure definitions.

use crate::parsers::parse_action_def;
use crate::types::StructureDef;
use nom::combinator::map;
use nom::IResult;

/// Parses a domain structure definition.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_structure_def};
/// # use pddl::types::{ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effect, GD, Literal, Name, PEffect, Predicate, Preference, PreferenceGD, PreGD, StructureDef, Term, Type, Typed, TypedList, Variable};
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
///         ActionSymbol::from_str("take-out"),
///         TypedList::from_iter([
///             Typed::new(Variable::from_str("x"), Type::Exactly("physob".into()))
///         ]),
///         Some(PreGD::Preference(PreferenceGD::from_gd(
///             GD::new_not(
///                 GD::new_atomic_formula(
///                     AtomicFormula::new_equality(
///                         Term::Variable(Variable::from_str("x")),
///                         Term::Name(Name::new("B"))
///                     )
///                 )
///             )
///         ))),
///         Some(Effect::new(CEffect::new_p_effect(
///             PEffect::NotAtomicFormula(
///                 AtomicFormula::new_predicate(
///                     Predicate::from_str("in"),
///                     vec![Term::Variable(Variable::from_str("x"))]
///                 )
///             )
///         )))
///     )
/// ))));
/// ```
pub fn parse_structure_def(input: &str) -> IResult<&str, StructureDef> {
    map(parse_action_def, StructureDef::new_action)(input)
}
