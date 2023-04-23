//! Provides parsers for conditional effects.

use crate::parsers::domain::parse_p_effect;
use crate::parsers::utility::{prefix_expr, space_separated_list0};
use crate::types::domain::ConditionalEffect;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parser combinator that parses conditional effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_cond_effect;
/// # use pddl::types::domain::{AtomicFormula, CEffect, ConditionalEffect, EqualityAtomicFormula, PEffect, Term};
/// assert_eq!(parse_cond_effect("(= x y)"), Ok(("",
///     ConditionalEffect::Single(
///         PEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_cond_effect("(and (= x y) (not (= ?a B)))"), Ok(("",
///     ConditionalEffect::All(vec![
///         PEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         ),
///         PEffect::NotAtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Variable("a".into()),
///                 Term::Name("B".into()))
///             )
///         )
///     ])
/// )));
/// ```
pub fn parse_cond_effect(input: &str) -> IResult<&str, ConditionalEffect> {
    let exactly = map(parse_p_effect, ConditionalEffect::from);
    let all = map(
        prefix_expr("and", space_separated_list0(parse_p_effect)),
        ConditionalEffect::from,
    );

    alt((exactly, all))(input)
}
