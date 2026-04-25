//! Provides parsers for atomic formulae.

use crate::parsers::{parens, space_separated_list0, ws};
use crate::parsers::{parse_predicate, ParseError, Span};
use crate::types::AtomicFormula;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::Parser;

/// Parses an atomic formula, i.e. `(<predicate> t*) | (= t t)`.
///
/// ## Example
/// ```
/// # use nom::character::complete::alpha1;
/// # use nom::Parser;
/// # use pddl::parsers::{atomic_formula, Span, parse_name, UnwrapValue};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, PredicateAtomicFormula, Predicate};
/// assert!(atomic_formula(parse_name).parse(Span::new("(= x y)")).is_value(
///     AtomicFormula::Equality(EqualityAtomicFormula::new("x".into(), "y".into()))
/// ));
/// assert!(atomic_formula(parse_name).parse(Span::new("(move a b)")).is_value(
///     AtomicFormula::Predicate(PredicateAtomicFormula::new(Predicate::from("move"), vec!["a".into(), "b".into()]))
/// ));
/// ```
pub fn atomic_formula<'a, F, O>(
    inner: F,
) -> impl Parser<Span<'a>, Output = AtomicFormula<O>, Error = ParseError<'a>>
where
    F: Clone + Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    let equality = map(
        delimited(
            ws(tag("(=")),
            (ws(inner.clone()), ws(inner.clone())),
            ws(char(')')),
        ),
        |tuple| AtomicFormula::Equality(tuple.into()),
    );

    let predicate = map(
        parens((parse_predicate, ws(space_separated_list0(inner)))),
        |tuple| AtomicFormula::Predicate(tuple.into()),
    );

    alt((equality, predicate))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{parse_name, parse_term, UnwrapValue};
    use crate::{EqualityAtomicFormula, Predicate, PredicateAtomicFormula};
    use nom::Parser;

    #[test]
    fn it_works() {
        let input = "(can-move ?from-waypoint ?to-waypoint)";
        let (_, _effect) = atomic_formula(parse_term).parse(Span::new(input)).unwrap();
    }

    #[test]
    fn test_parse() {
        assert!(atomic_formula(parse_name)
            .parse(Span::new("(= x y)"))
            .is_value(AtomicFormula::Equality(EqualityAtomicFormula::new(
                "x".into(),
                "y".into()
            ))));
        assert!(atomic_formula(parse_name)
            .parse(Span::new("(move a b)"))
            .is_value(AtomicFormula::Predicate(PredicateAtomicFormula::new(
                Predicate::from("move"),
                vec!["a".into(), "b".into()]
            ))));
    }
}
