//! Provides parsers for predicate definitions.

use crate::parsers::domain::parse_atomic_formula_skeleton;
use crate::parsers::utility::{prefix_expr, space_separated_list1};
use crate::types::domain::PredicateDefinitions;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses predicate definitions, i.e. `(:predicates <atomic formula skeleton>⁺)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_predicates_def;
/// # use pddl::types::domain::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions};
/// # use pddl::types::utility::{ToTyped, TypedList};
///
/// let input = r#"(:predicates
///                     (at ?x - physob ?y - location)
///                     (in ?x ?y - physob)
///                )"#;
///
/// assert_eq!(parse_predicates_def(input), Ok(("",
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
/// )));
/// ```
pub fn parse_predicates_def(input: &str) -> IResult<&str, PredicateDefinitions> {
    map(
        prefix_expr(
            ":predicates",
            space_separated_list1(parse_atomic_formula_skeleton),
        ),
        |vec| PredicateDefinitions::new(vec),
    )(input)
}
