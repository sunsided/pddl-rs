//! Provides parsers for conditional effects.

use nom::branch::alt;
use nom::combinator::map;

use crate::parsers::{parse_p_effect, ParseResult, Span};
use crate::parsers::{prefix_expr, space_separated_list0};
use crate::types::ConditionalEffect;

/// Parses conditional effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_cond_effect, preamble::*};
/// # use pddl::{AtomicFormula, CEffect, ConditionalEffect, EqualityAtomicFormula, PEffect, Term};
/// assert!(parse_cond_effect("(= x y)").is_value(
///     ConditionalEffect::Single(
///         PEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         )
///     )
/// ));
///
/// assert!(parse_cond_effect("(and (= x y) (not (= ?a B)))").is_value(
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
/// ));
/// ```
pub fn parse_cond_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, ConditionalEffect> {
    let exactly = map(parse_p_effect, ConditionalEffect::from);
    let all = map(
        prefix_expr("and", space_separated_list0(parse_p_effect)),
        ConditionalEffect::from,
    );

    alt((all, exactly))(input.into())
}

impl crate::parsers::Parser for ConditionalEffect {
    type Item = ConditionalEffect;

    /// See [`parse_cond_effect`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_cond_effect(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{AtomicFormula, ConditionalEffect, EqualityAtomicFormula, PEffect, Parser, Term};

    #[test]
    fn test_parse() {
        assert!(
            ConditionalEffect::parse("(= x y)").is_value(ConditionalEffect::Single(
                PEffect::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                )))
            ))
        );

        assert!(
            ConditionalEffect::parse("(and (= x y) (not (= ?a B)))").is_value(
                ConditionalEffect::All(vec![
                    PEffect::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    PEffect::NotAtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                        Term::Variable("a".into()),
                        Term::Name("B".into())
                    )))
                ])
            )
        );
    }
}
