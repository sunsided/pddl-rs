//! Provides parsers for p-effects.

use crate::parsers::{atomic_formula, parse_term, prefix_expr};
use crate::types::PEffect;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parser combinator that parses p-effects.
///
/// ## Example
/// ```
/// use pddl::parsers::parse_p_effect;
/// use pddl::types::{AtomicFormula, EqualityAtomicFormula, PEffect, Term};
/// assert_eq!(parse_p_effect("(= x y)"), Ok(("",
///     PEffect::AtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Name("x".into()),
///             Term::Name("y".into()))
///         )
///     )
/// )));
/// assert_eq!(parse_p_effect("(not (= ?a B))"), Ok(("",
///     PEffect::NotAtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Variable("a".into()),
///             Term::Name("B".into()))
///         )
///     )
/// )));
/// ```
pub fn parse_p_effect(input: &str) -> IResult<&str, PEffect> {
    let is = map(atomic_formula(parse_term), |af| PEffect::new(af));
    let is_not = map(prefix_expr("not", atomic_formula(parse_term)), |af| {
        PEffect::new_not(af)
    });

    // TODO: Add :numeric-fluents and :object-fluents variants.

    alt((is_not, is))(input)
}
