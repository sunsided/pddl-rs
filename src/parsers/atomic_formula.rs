//! Provides parsers for atomic formulae.

use crate::parsers::{parens, parse_predicate, space_separated_list0, ws};
use crate::types::AtomicFormula;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace1};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

/// Parser combinator that parses an atomic formula, i.e. `(<predicate> t*) | (= t t)`.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use pddl::parsers::{parse_name, atomic_formula};
/// # use pddl::types::{AtomicFormula, EqualityAtomicFormula, Name, PredicateAtomicFormula, PrimitiveType, Type, Typed, Predicate};
///
/// assert_eq!(atomic_formula(parse_name)("(= x y)"), Ok(("",
///     AtomicFormula::Equality(EqualityAtomicFormula::new("x".into(), "y".into()))
/// )));
/// assert_eq!(atomic_formula(parse_name)("(move a b)"), Ok(("",
///     AtomicFormula::Predicate(PredicateAtomicFormula::new(Predicate::from("move"), vec!["a".into(), "b".into()]))
/// )));
/// ```
pub fn atomic_formula<'a, F, O>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, AtomicFormula<O>>
where
    F: Clone + FnMut(&'a str) -> IResult<&'a str, O>,
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
