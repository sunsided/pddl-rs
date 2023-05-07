//! Provides parsers for derived predicates.

use crate::parsers::{parse_atomic_formula_skeleton, parse_gd};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::DerivedPredicate;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses a derived predicate, i.e. `(:derived <atomic formula skeleton> <GD>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_derived_predicate, preamble::*};
/// # use pddl::GoalDefinition;
/// let input = r#"(:derived (train-usable ?t - train)
///                     (and
///                         (train-has-guard ?t)
///                         (train-has-driver ?t)
///                     )
///                 )"#;
///
/// let (remaining, predicate) = parse_derived_predicate(input).unwrap();
/// assert_eq!(predicate.predicate().name(), "train-usable");
/// assert!(matches!(predicate.expression(), &GoalDefinition::And(..)));
///```
pub fn parse_derived_predicate<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, DerivedPredicate> {
    map(
        prefix_expr(
            ":derived",
            tuple((
                parse_atomic_formula_skeleton,
                preceded(multispace1, parse_gd),
            )),
        ),
        DerivedPredicate::from,
    )(input.into())
}

impl crate::parsers::Parser for DerivedPredicate {
    type Item = DerivedPredicate;

    /// See [`parse_derived_predicate`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_derived_predicate(input)
    }
}
