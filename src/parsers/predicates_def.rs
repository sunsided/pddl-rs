//! Provides parsers for predicate definitions.

use crate::parsers::{parse_atomic_formula_skeleton, ParseResult, Span};
use crate::parsers::{prefix_expr, space_separated_list1};
use crate::types::PredicateDefinitions;
use nom::combinator::map;

/// Parser that parses predicate definitions, i.e. `(:predicates <atomic formula skeleton>⁺)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_predicates_def, preamble::*};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions};
/// # use pddl::{ToTyped, TypedList};
/// let input = r#"(:predicates
///                     (at ?x - physob ?y - location)
///                     (in ?x ?y - physob)
///                )"#;
///
/// assert!(parse_predicates_def(input).is_value(
///     PredicateDefinitions::new(vec![
///         AtomicFormulaSkeleton::new(
///             Predicate::from("at"),
///             TypedList::from_iter([
///                 Variable::from("x").to_typed("physob"),
///                 Variable::from("y").to_typed("location"),
///             ])),
///         AtomicFormulaSkeleton::new(
///             Predicate::from("in"),
///             TypedList::from_iter([
///                 Variable::from("x").to_typed("physob"),
///                 Variable::from("y").to_typed("physob"),
///             ]))
///     ])
/// ));
/// ```
pub fn parse_predicates_def<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, PredicateDefinitions<'a>> {
    map(
        prefix_expr(
            ":predicates",
            space_separated_list1(parse_atomic_formula_skeleton),
        ),
        |vec| PredicateDefinitions::new(vec),
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for PredicateDefinitions<'a> {
    type Item = PredicateDefinitions<'a>;

    /// See [`parse_predicates_def`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_predicates_def(input.into())
    }
}
