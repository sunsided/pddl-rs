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
/// let (remaining, predicate) = parse_derived_predicate(input.into()).unwrap();
/// assert_eq!(predicate.predicate().name(), &"train-usable".into());
/// assert!(matches!(predicate.expression(), &GoalDefinition::And(..)));
///```
pub fn parse_derived_predicate(input: Span) -> ParseResult<DerivedPredicate> {
    map(
        prefix_expr(
            ":derived",
            tuple((
                parse_atomic_formula_skeleton,
                preceded(multispace1, parse_gd),
            )),
        ),
        DerivedPredicate::from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for DerivedPredicate<'a> {
    type Item = DerivedPredicate<'a>;

    /// See [`parse_derived_predicate`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_derived_predicate(input)
    }
}
