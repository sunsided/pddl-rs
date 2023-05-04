//! Provides parsers for atomic formulae.

use crate::parsers::{parens, space_separated_list0, ws};
use crate::parsers::{parse_predicate, ParseResult, Span};
use crate::types::AtomicFormula;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace1};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};

/// Parses an atomic formula, i.e. `(<predicate> t*) | (= t t)`.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use pddl::parsers::{atomic_formula, Span, parse_name, UnwrapValue};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, PredicateAtomicFormula, Predicate};
/// assert!(atomic_formula(parse_name)(Span::new("(= x y)")).is_value(
///     AtomicFormula::Equality(EqualityAtomicFormula::new("x".into(), "y".into()))
/// ));
/// assert!(atomic_formula(parse_name)(Span::new("(move a b)")).is_value(
///     AtomicFormula::Predicate(PredicateAtomicFormula::new(Predicate::from("move"), vec!["a".into(), "b".into()]))
/// ));
/// ```
pub fn atomic_formula<'a, F, O>(
    inner: F,
) -> impl FnMut(Span<'a>) -> ParseResult<'a, AtomicFormula<O>>
where
    F: Clone + FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    let equality = map(
        delimited(
            tag("(="),
            preceded(
                multispace1,
                tuple((inner.clone(), preceded(multispace1, inner.clone()))),
            ),
            char(')'),
        ),
        |tuple| AtomicFormula::Equality(tuple.into()),
    );

    let predicate = map(
        parens(tuple((parse_predicate, ws(space_separated_list0(inner))))),
        |tuple| AtomicFormula::Predicate(tuple.into()),
    );

    alt((equality, predicate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::parse_term;

    #[test]
    fn it_works() {
        let input = "(can-move ?from-waypoint ?to-waypoint)";
        let (_, _effect) = atomic_formula(parse_term)(Span::new(input)).unwrap();
    }
}
