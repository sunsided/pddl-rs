//! Provides parsers for effect conditions.

use nom::branch::alt;
use nom::combinator::map;
use nom::Parser;

use crate::parsers::{parse_p_effect, ParseResult, Span};
use crate::parsers::{prefix_expr, space_separated_list0};
use crate::types::EffectCondition;

/// Parses effect conditions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_effect_condition, preamble::*};
/// # use pddl::{AtomicFormula, EffectCondition, EqualityAtomicFormula, PrimitiveEffect, Term};
/// assert!(parse_effect_condition("(= x y)").is_value(
///     EffectCondition::Single(
///         PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         )
///     )
/// ));
///
/// assert!(parse_effect_condition("(and (= x y) (not (= ?a B)))").is_value(
///     EffectCondition::All(vec![
///         PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         ),
///         PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Variable("a".into()),
///                 Term::Name("B".into()))
///             )
///         )
///     ])
/// ));
/// ```
#[allow(deprecated)]
pub fn parse_effect_condition<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, EffectCondition> {
    let exactly = map(parse_p_effect, EffectCondition::from);
    let all = map(
        prefix_expr("and", space_separated_list0(parse_p_effect)),
        EffectCondition::from,
    );

    alt((all, exactly)).parse(input.into())
}

/// Alias for [`parse_effect_condition`].
#[deprecated(since = "0.2.0", note = "Use `parse_effect_condition` instead")]
pub fn parse_cond_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, EffectCondition> {
    parse_effect_condition(input)
}

impl crate::parsers::Parser for EffectCondition {
    type Item = EffectCondition;

    /// See [`parse_effect_condition`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_effect_condition(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{AtomicFormula, EffectCondition, EqualityAtomicFormula, PrimitiveEffect, Parser, Term};

    #[test]
    fn test_parse() {
        assert!(
            EffectCondition::parse("(= x y)").is_value(EffectCondition::Single(
                PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                )))
            ))
        );

        assert!(
            EffectCondition::parse("(and (= x y) (not (= ?a B)))").is_value(
                EffectCondition::All(vec![
                    PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                        Term::Variable("a".into()),
                        Term::Name("B".into())
                    )))
                ])
            )
        );
    }
}
