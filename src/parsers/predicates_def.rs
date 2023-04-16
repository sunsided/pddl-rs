//! Provides parsers for predicate definitions.

use crate::parsers::{parse_atomic_formula_skeleton, prefix_expr, space_separated_list1};
use crate::types::PredicateDefinitions;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses predicate definitions, i.e. `(:predicates <atomic formula skeleton>⁺)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_predicates_def;
/// # use pddl::types::{Variable, AtomicFormulaSkeleton, Predicate, Typed, Type, PredicateDefinitions};
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
///             vec![
///                 Typed::new(Variable::from("x"), Type::Exactly("physob".into())),
///                 Typed::new(Variable::from("y"), Type::Exactly("location".into()))
///             ]),
///         AtomicFormulaSkeleton::new(
///             Predicate::from("in"),
///             vec![
///                 Typed::new(Variable::from("x"), Type::Exactly("physob".into())),
///                 Typed::new(Variable::from("y"), Type::Exactly("physob".into()))
///             ])
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
