//! Provides parsers for literals.

use nom::branch::alt;
use nom::combinator::map;
use nom::Parser;

use crate::parsers::prefix_expr;
use crate::parsers::{atomic_formula, ParseError, ParseResult, Span};
use crate::types::Literal;

/// Parser combinator that parses a literal, i.e. `<atomic formula(t)> | (not <atomic formula(t)>)`.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use nom::Parser;
/// # use pddl::parsers::{literal, parse_name, preamble::*};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, PredicateAtomicFormula, Predicate, Literal};
/// assert!(literal(parse_name).parse(Span::new("(= x y)")).is_value(
///     Literal::AtomicFormula(
///         AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 "x".into(),
///                 "y".into()
///             )
///         )
///     )
/// ));
/// assert!(literal(parse_name).parse(Span::new("(not (= x y))")).is_value(
///     Literal::NotAtomicFormula(
///         AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 "x".into(),
///                 "y".into()
///             )
///         )
///     )
/// ));
/// ```
pub fn literal<'a, F, O>(
    inner: F,
) -> impl Parser<Span<'a>, Output = Literal<O>, Error = ParseError<'a>>
where
    F: Clone + Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    let is = map(atomic_formula(inner.clone()), |af| Literal::new(af));
    let is_not = map(prefix_expr("not", atomic_formula(inner)), |af| {
        Literal::new_not(af)
    });

    alt((is_not, is))
}

#[cfg(test)]
mod tests {
    use crate::parsers::{literal, parse_name, Span, UnwrapValue};
    use crate::{AtomicFormula, EqualityAtomicFormula, Literal};
    use nom::Parser;

    #[test]
    fn test_parse() {
        assert!(literal(parse_name)
            .parse(Span::new("(= x y)"))
            .is_value(Literal::AtomicFormula(AtomicFormula::Equality(
                EqualityAtomicFormula::new("x".into(), "y".into())
            ))));
        assert!(literal(parse_name)
            .parse(Span::new("(not (= x y))"))
            .is_value(Literal::NotAtomicFormula(AtomicFormula::Equality(
                EqualityAtomicFormula::new("x".into(), "y".into())
            ))));
    }
}
