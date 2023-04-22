//! Provides parsers for durative action effects.

use crate::parsers::{
    parens, parse_da_gd, parse_timed_effect, parse_variable, prefix_expr, space_separated_list0,
    typed_list,
};
use crate::types::DurativeActionEffect;
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
/// # use pddl::types::{AtomicFormula, ConditionalEffect, DurativeActionEffect, EqualityAtomicFormula, PEffect, Term, TimedEffect, TimeSpecifier, Typed, TypedList, Variable};
/// assert_eq!(parse_da_effect("(at start (= x y))"), Ok(("",
///     DurativeActionEffect::Timed(
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
///     DurativeActionEffect::new_and([])
/// )));
///
/// assert_eq!(parse_da_effect("(and (at start (= x y)) (and ))"), Ok(("",
///     DurativeActionEffect::new_and([
///         DurativeActionEffect::Timed(
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
///         DurativeActionEffect::new_and([])
///     ])
/// )));
///
/// assert_eq!(parse_da_effect("(forall (?a ?b) (at start (= a b)))"), Ok(("",
///     DurativeActionEffect::new_forall(
///         TypedList::from_iter([
///             Typed::new_object(Variable::from_str("a")),
///             Typed::new_object(Variable::from_str("b")),
///         ]),
///         DurativeActionEffect::Timed(
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
pub fn parse_da_effect(input: &str) -> IResult<&str, DurativeActionEffect> {
    let exactly = map(parse_timed_effect, DurativeActionEffect::from);

    let all = map(
        prefix_expr("and", space_separated_list0(parse_da_effect)),
        DurativeActionEffect::from_iter,
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
        DurativeActionEffect::from,
    );

    // :conditional-effects
    let when = map(
        prefix_expr(
            "when",
            tuple((parse_da_gd, preceded(multispace1, parse_timed_effect))),
        ),
        DurativeActionEffect::from,
    );

    alt((all, forall, when, exactly))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_works() {
        let input = r#"(and
                (decrease (fuel-level ?t) (* 2 #t))
                (at end (at ?rover ?to-waypoint))
                (at end (been-at ?rover ?to-waypoint))
                (at start (not (at ?rover ?from-waypoint)))
                (at start (decrease (battery-amount ?rover) 8))
                (at end (increase (distance-travelled) 5))
                )"#;

        let (_, _effect) = parse_da_effect(input).unwrap();
    }

    #[test]
    fn complex_works() {
        let input = r#"(and
                (decrease (fuel-level ?t) (* 2 #t))
                (at end (at ?rover ?to-waypoint))
                (at end (been-at ?rover ?to-waypoint))
                (at start (not (at ?rover ?from-waypoint)))
                (at start (decrease (battery-amount ?rover) 8))
                (at end (increase (distance-travelled) 5))
                )"#;

        let (_, _effect) = parse_da_effect(input).unwrap();
    }
}
