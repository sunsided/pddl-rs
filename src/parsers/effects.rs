//! Provides parsers for effects.

use nom::branch::alt;
use nom::combinator::map;

use crate::parsers::{parse_c_effect, ParseResult, Span};
use crate::parsers::{prefix_expr, space_separated_list0};
use crate::types::Effects;

/// Parser for effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_effect, preamble::*};
/// # use pddl::{AtomicFormula, CEffect, Effects, EqualityAtomicFormula, PEffect, Term};
/// assert!(parse_effect("(= x y)").is_value(
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
/// assert!(parse_effect("(and (= x y) (not (= ?a B)))").is_value(
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
pub fn parse_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Effects> {
    let exactly = map(parse_c_effect, Effects::from);
    let all = map(
        prefix_expr("and", space_separated_list0(parse_c_effect)),
        Effects::from,
    );

    alt((all, exactly))(input.into())
}

impl crate::parsers::Parser for Effects {
    type Item = Effects;

    /// See [`parse_effect`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_effect(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{AtomicFormula, CEffect, Effects, EqualityAtomicFormula, PEffect, Parser, Term};

    #[test]
    fn test_parse() {
        assert!(
            Effects::parse("(= x y)").is_value(Effects::new(CEffect::Effect(
                PEffect::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                )))
            )))
        );
        assert!(
            Effects::parse("(and (= x y) (not (= ?a B)))").is_value(Effects::from_iter([
                CEffect::Effect(PEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                ))),
                CEffect::Effect(PEffect::NotAtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Variable("a".into()), Term::Name("B".into()))
                )))
            ]))
        );
    }
}
