//! Provides parsers for c-effects.

use crate::parsers::{parens, prefix_expr, typed_list};
use crate::parsers::{parse_cond_effect, parse_effect, parse_gd, parse_p_effect, parse_variable};
use crate::types::CEffect;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser that parses c-effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_c_effect;
/// # use pddl::types::{AtomicFormula, CEffect, Effect, EqualityAtomicFormula, PEffect, Term, Variable};
/// # use pddl::types::{Typed, TypedList};
/// assert_eq!(parse_c_effect("(= x y)"), Ok(("",
///     CEffect::Effect(
///         PEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         )
///     )
/// )));
/// assert_eq!(parse_c_effect("(not (= ?a B))"), Ok(("",
///     CEffect::Effect(
///         PEffect::NotAtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Variable("a".into()),
///                 Term::Name("B".into()))
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_c_effect("(forall (?a ?b) (= ?a ?b))"), Ok(("",
///     CEffect::new_forall(
///         TypedList::from_iter([
///             Typed::new_object(Variable::from_str("a")),
///             Typed::new_object(Variable::from_str("b")),
///         ]),
///         Effect::new(CEffect::Effect(
///             PEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Variable("a".into()),
///                     Term::Variable("b".into()))
///                 )
///             )
///         ))
///     )
/// )));
/// ```
pub fn parse_c_effect(input: &str) -> IResult<&str, CEffect> {
    let p_effect = map(parse_p_effect, CEffect::from);
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_effect),
            )),
        ),
        CEffect::from,
    );
    let when = map(
        prefix_expr(
            "when",
            tuple((parse_gd, preceded(multispace1, parse_cond_effect))),
        ),
        CEffect::from,
    );

    alt((forall, when, p_effect))(input)
}
