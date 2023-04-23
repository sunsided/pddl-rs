//! Provides parsers for literals.

use crate::parsers::atomic_formula;
use crate::parsers::prefix_expr;
use crate::types::Literal;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parser combinator that parses a literal, i.e. `<atomic formula(t)> | (not <atomic formula(t)>)`.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use pddl::parsers::literal;
/// # use pddl::parsers::parse_name;
/// # use pddl::types::{AtomicFormula, EqualityAtomicFormula, PredicateAtomicFormula, Predicate, Literal};
///
/// assert_eq!(literal(parse_name)("(= x y)"), Ok(("",
///     Literal::AtomicFormula(
///         AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 "x".into(),
///                 "y".into()
///             )
///         )
///     )
/// )));
/// assert_eq!(literal(parse_name)("(not (= x y))"), Ok(("",
///     Literal::NotAtomicFormula(
///         AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 "x".into(),
///                 "y".into()
///             )
///         )
///     )
/// )));
/// ```
pub fn literal<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, Literal<O>>
where
    F: Clone + FnMut(&'a str) -> IResult<&'a str, O>,
{
    let is = map(atomic_formula(inner.clone()), |af| Literal::new(af));
    let is_not = map(prefix_expr("not", atomic_formula(inner)), |af| {
        Literal::new_not(af)
    });

    alt((is_not, is))
}
