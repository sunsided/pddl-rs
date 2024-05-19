//! Provides parsers for action definitions.

use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::{map, opt};
use nom::sequence::{preceded, tuple};

use crate::parsers::{empty_or, parens, prefix_expr, typed_list, ws, ParseResult, Span};
use crate::parsers::{parse_action_symbol, parse_effect, parse_pre_gd, parse_variable};
use crate::types::ActionDefinition;

/// Parses an action definition.
///
/// ## Example
/// ```
/// # use pddl::{ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effects, GoalDefinition, Name, PEffect, Predicate, PreferenceGD, PreconditionGoalDefinitions, PreconditionGoalDefinition, Term, ToTyped, TypedList, Variable};
/// # use pddl::parsers::{parse_action_def, Span, UnwrapValue};
/// let input = r#"(:action take-out
///                     :parameters (?x - physob)
///                     :precondition (not (= ?x B))
///                     :effect (not (in ?x))
///                 )"#;
///
/// let action = parse_action_def(Span::new(input));
///
/// assert!(action.is_value(
///     ActionDefinition::new(
///         ActionSymbol::from_str("take-out"),
///         TypedList::from_iter([
///             Variable::from_str("x").to_typed("physob")
///         ]),
///         PreconditionGoalDefinitions::from(
///             PreconditionGoalDefinition::Preference(PreferenceGD::from_gd(
///                 GoalDefinition::new_not(
///                     GoalDefinition::AtomicFormula(
///                         AtomicFormula::new_equality(
///                             Term::Variable(Variable::from_str("x")),
///                             Term::Name(Name::new("B"))
///                         )
///                     )
///                 )
///             )
///         )),
///         Some(Effects::new(CEffect::new_p_effect(
///             PEffect::NotAtomicFormula(
///                 AtomicFormula::new_predicate(
///                     Predicate::from_str("in"),
///                     vec![Term::Variable(Variable::from_str("x"))]
///                 )
///             )
///         )))
///     )
/// ));
/// ```
pub fn parse_action_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, ActionDefinition> {
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
        ActionDefinition::new(
            symbol,
            params,
            preconditions.flatten().into(),
            effects.flatten(),
        )
    })(input.into())
}

impl crate::parsers::Parser for ActionDefinition {
    type Item = ActionDefinition;

    /// Parses an action definition.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effects, GoalDefinition, Name, PEffect, Predicate, PreferenceGD, PreconditionGoalDefinitions, PreconditionGoalDefinition, Term, ToTyped, TypedList, Variable, Parser};
    /// # use pddl::parsers::{parse_action_def, Span, UnwrapValue};
    /// let input = r#"(:action take-out
    ///                     :parameters (?x - physob)
    ///                     :precondition (not (= ?x B))
    ///                     :effect (not (in ?x))
    ///                 )"#;
    ///
    /// let (_, action) = ActionDefinition::parse(input).unwrap();
    ///
    /// assert_eq!(action,
    ///     ActionDefinition::new(
    ///         ActionSymbol::from_str("take-out"),
    ///         TypedList::from_iter([
    ///             Variable::from_str("x").to_typed("physob")
    ///         ]),
    ///         PreconditionGoalDefinitions::from_str("(not (= ?x B))").unwrap(),
    ///         Some(Effects::from_str("(not (in ?x))").unwrap())
    ///     )
    /// );
    /// ```
    ///
    /// ## See also
    /// See [`parse_action_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_action_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ActionDefinition, ActionSymbol, Effects, Parser, PreconditionGoalDefinitions, ToTyped,
        TypedList, Variable,
    };

    #[test]
    fn test_parse() {
        let input = r#"(:action take-out
                            :parameters (?x - physob)
                            :precondition (not (= ?x B))
                            :effect (not (in ?x))
                        )"#;

        let (_, action) = ActionDefinition::parse(input).unwrap();

        assert_eq!(
            action,
            ActionDefinition::new(
                ActionSymbol::from_str("take-out"),
                TypedList::from_iter([Variable::from_str("x").to_typed("physob")]),
                PreconditionGoalDefinitions::from_str("(not (= ?x B))").unwrap(),
                Some(Effects::from_str("(not (in ?x))").unwrap())
            )
        );
    }
}
