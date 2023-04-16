//! Provides parsers for c-effects.

use crate::parsers::parse_p_effect;
use crate::types::CEffect;
use nom::combinator::map;
use nom::IResult;

/// Parser combinator that parses c-effects.
///
/// ## Example
/// ```
/// use pddl::parsers::parse_c_effect;
/// use pddl::types::{AtomicFormula, CEffect, EqualityAtomicFormula, PEffect, Term};
/// assert_eq!(parse_c_effect("(= x y)"), Ok(("",
///     CEffect::PEffect(
///         PEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         )
///     )
/// )));
/// assert_eq!(parse_c_effect("(not (= ?a B))"), Ok(("",
///     CEffect::PEffect(
///         PEffect::NotAtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Variable("a".into()),
///                 Term::Name("B".into()))
///             )
///         )
///     )
/// )));
/// ```
pub fn parse_c_effect(input: &str) -> IResult<&str, CEffect> {
    map(parse_p_effect, CEffect::from)(input)

    // TODO: Add :conditional-effects variants
}
