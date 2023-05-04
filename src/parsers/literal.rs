//! Provides parsers for literals.

use crate::parsers::prefix_expr;
use crate::parsers::{atomic_formula, ParseResult, Span};
use crate::types::Literal;
use nom::branch::alt;
use nom::combinator::map;

/// Parser combinator that parses a literal, i.e. `<atomic formula(t)> | (not <atomic formula(t)>)`.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use pddl::parsers::{literal, parse_name, preamble::*};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, PredicateAtomicFormula, Predicate, Literal};
/// assert!(literal(parse_name)(Span::new("(= x y)")).is_value(
///     Literal::AtomicFormula(
///         AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 "x".into(),
///                 "y".into()
///             )
///         )
///     )
/// ));
/// assert!(literal(parse_name)(Span::new("(not (= x y))")).is_value(
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
pub fn literal<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, Literal<O>>
where
    F: Clone + FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    let is = map(atomic_formula(inner.clone()), |af| Literal::new(af));
    let is_not = map(prefix_expr("not", atomic_formula(inner)), |af| {
        Literal::new_not(af)
    });

    alt((is_not, is))
}
