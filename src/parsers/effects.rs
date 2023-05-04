//! Provides parsers for effects.

use crate::parsers::{parse_c_effect, ParseResult, Span};
use crate::parsers::{prefix_expr, space_separated_list0};
use crate::types::Effects;
use nom::branch::alt;
use nom::combinator::map;

/// Parser for effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_effect, preamble::*};
/// # use pddl::{AtomicFormula, CEffect, Effects, EqualityAtomicFormula, PEffect, Term};
/// assert!(parse_effect("(= x y)".into()).is_value(
///     Effects::new(
///         CEffect::Effect(
///             PEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Name("x".into()),
///                     Term::Name("y".into()))
///                 )
///             )
///         )
///     )
/// ));
/// assert!(parse_effect("(and (= x y) (not (= ?a B)))".into()).is_value(
///     Effects::from_iter([
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
/// ));
/// ```
pub fn parse_effect<'a>(input: Span<'a>) -> ParseResult<'a, Effects> {
    let exactly = map(parse_c_effect, Effects::from);
    let all = map(
        prefix_expr("and", space_separated_list0(parse_c_effect)),
        Effects::from,
    );

    alt((exactly, all))(input)
}

impl<'a> crate::parsers::Parser<'a> for Effects<'a> {
    type Item = Effects<'a>;

    /// See [`parse_effect`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_effect(input)
    }
}
