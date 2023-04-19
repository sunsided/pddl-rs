//! Provides parsers for action definitions.

use crate::parsers::{
    empty_or, parens, parse_action_symbol, parse_effect, parse_pre_gd, parse_variable, prefix_expr,
    typed_list, ws,
};
use crate::types::ActionDefinition;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses an action definition.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_action_def;
/// # use pddl::types::{ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effect, GD, Literal, Name, PEffect, Predicate, Preference, PreferenceGD, PreGD, Term, Type, Typed, TypedList, Variable};
///
/// let input = r#"(:action take-out
///                     :parameters (?x - physob)
///                     :precondition (not (= ?x B))
///                     :effect (not (in ?x))
///                 )"#;
///
/// let action = parse_action_def(input);
///
/// assert_eq!(action, Ok(("",
///     ActionDefinition::new(
///         ActionSymbol::from_str("take-out"),
///         TypedList::from_iter([
///             Typed::new(Variable::from_str("x"), Type::Exactly("physob".into()))
///         ]),
///         Some(PreGD::Preference(PreferenceGD::from_gd(
///             GD::new_not(
///                 GD::AtomicFormula(
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
/// )));
/// ```
pub fn parse_action_def(input: &str) -> IResult<&str, ActionDefinition> {
    let precondition = preceded(
        tag(":precondition"),
        preceded(multispace1, empty_or(parse_pre_gd)),
    );
    let effect = preceded(
        tag(":effect"),
        preceded(multispace1, empty_or(parse_effect)),
    );
    let action_def_body = tuple((opt(ws(precondition)), opt(ws(effect))));
    let parameters = preceded(
        tag(":parameters"),
        preceded(multispace1, parens(typed_list(parse_variable))),
    );
    let action_def = prefix_expr(
        ":action",
        tuple((
            parse_action_symbol,
            preceded(multispace1, parameters),
            ws(action_def_body),
        )),
    );

    map(action_def, |(symbol, params, (preconditions, effects))| {
        ActionDefinition::new(symbol, params, preconditions.flatten(), effects.flatten())
    })(input)
}
