//! Provides parsers for durative action effects.

use crate::parsers::{
    parens, parse_da_gd, parse_timed_effect, parse_variable, prefix_expr, space_separated_list0,
    typed_list,
};
use crate::types::DAEffect;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser combinator that parses effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_da_effect;
/// # use pddl::types::{AtomicFormula, ConditionalEffect, DAEffect, EqualityAtomicFormula, PEffect, Term, TimedEffect, TimeSpecifier, Typed, TypedList, Variable};
/// assert_eq!(parse_da_effect("(at start (= x y))"), Ok(("",
///     DAEffect::Timed(
///         TimedEffect::new_conditional(
///             TimeSpecifier::Start,
///             ConditionalEffect::new(
///                 PEffect::AtomicFormula(AtomicFormula::Equality(
///                     EqualityAtomicFormula::new(
///                         Term::Name("x".into()),
///                         Term::Name("y".into()))
///                     )
///                 )
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_da_effect("(and )"), Ok(("",
///     DAEffect::new_and([])
/// )));
///
/// assert_eq!(parse_da_effect("(and (at start (= x y)) (and ))"), Ok(("",
///     DAEffect::new_and([
///         DAEffect::Timed(
///             TimedEffect::new_conditional(
///                 TimeSpecifier::Start,
///                 ConditionalEffect::new(
///                     PEffect::AtomicFormula(AtomicFormula::Equality(
///                         EqualityAtomicFormula::new(
///                             Term::Name("x".into()),
///                             Term::Name("y".into()))
///                         )
///                     )
///                 )
///             )
///         ),
///         DAEffect::new_and([])
///     ])
/// )));
///
/// assert_eq!(parse_da_effect("(forall (?a ?b) (at start (= a b)))"), Ok(("",
///     DAEffect::new_forall(
///         TypedList::from_iter([
///             Typed::new_object(Variable::from_str("a")),
///             Typed::new_object(Variable::from_str("b")),
///         ]),
///         DAEffect::Timed(
///             TimedEffect::new_conditional(
///                 TimeSpecifier::Start,
///                 ConditionalEffect::new(
///                     PEffect::AtomicFormula(AtomicFormula::Equality(
///                         EqualityAtomicFormula::new(
///                             Term::Name("a".into()),
///                             Term::Name("b".into()))
///                         )
///                     )
///                 )
///             )
///         )
///     )
/// )));
/// ```
pub fn parse_da_effect(input: &str) -> IResult<&str, DAEffect> {
    let exactly = map(parse_timed_effect, DAEffect::from);
    let all = map(
        prefix_expr("and", space_separated_list0(parse_da_effect)),
        DAEffect::from_iter,
    );

    // :conditional-effects
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_da_effect),
            )),
        ),
        DAEffect::from,
    );

    // :conditional-effects
    let when = map(
        prefix_expr(
            "when",
            tuple((parse_da_gd, preceded(multispace1, parse_timed_effect))),
        ),
        DAEffect::from,
    );

    alt((all, exactly, forall, when))(input)
}
