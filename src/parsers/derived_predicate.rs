//! Provides parsers for derived predicates.

use crate::parsers::prefix_expr;
use crate::parsers::{parse_atomic_formula_skeleton, parse_gd};
use crate::types::DerivedPredicate;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses a derived predicate, i.e. `(:derived <atomic formula skeleton> <GD>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_derived_predicate;
/// # use pddl::types::GoalDefinition;
///
/// let input = r#"(:derived (train-usable ?t - train)
///                     (and
///                         (train-has-guard ?t)
///                         (train-has-driver ?t)
///                     )
///                 )"#;
///
/// let (remaining, predicate) = parse_derived_predicate(input).unwrap();
/// assert_eq!(predicate.predicate().name(), &"train-usable".into());
/// assert!(matches!(predicate.expression(), &GoalDefinition::And(..)));
///```
pub fn parse_derived_predicate(input: &str) -> IResult<&str, DerivedPredicate> {
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

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_derived_predicate(input)
    }
}
