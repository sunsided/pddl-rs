//! Provides parsers for effects.

use crate::parsers::parse_c_effect;
use crate::parsers::{prefix_expr, space_separated_list0};
use crate::types::Effect;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parser for effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_effect;
/// # use pddl::{AtomicFormula, CEffect, Effect, EqualityAtomicFormula, PEffect, Term};
/// assert_eq!(parse_effect("(= x y)"), Ok(("",
///     Effect::new(
///         CEffect::Effect(
///             PEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Name("x".into()),
///                     Term::Name("y".into()))
///                 )
///             )
///         )
///     )
/// )));
/// assert_eq!(parse_effect("(and (= x y) (not (= ?a B)))"), Ok(("",
///     Effect::from_iter([
///         CEffect::Effect(
///             PEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Name("x".into()),
///                     Term::Name("y".into()))
///                 )
///             )
///         ),
///         CEffect::Effect(
///             PEffect::NotAtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Variable("a".into()),
///                     Term::Name("B".into()))
///                 )
///             )
///         )
///     ])
/// )));
/// ```
pub fn parse_effect(input: &str) -> IResult<&str, Effect> {
    let exactly = map(parse_c_effect, Effect::from);
    let all = map(
        prefix_expr("and", space_separated_list0(parse_c_effect)),
        Effect::from,
    );

    alt((exactly, all))(input)
}

impl<'a> crate::parsers::Parser<'a> for Effect<'a> {
    type Item = Effect<'a>;

    /// See [`parse_effect`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_effect(input)
    }
}
