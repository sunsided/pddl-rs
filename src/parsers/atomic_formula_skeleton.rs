//! Provides parsers for atomic formula skeletons.

use crate::parsers::{parens, parse_predicate, parse_variable, typed_list, ws};
use crate::types::AtomicFormulaSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

/// Parser that parses an atomic formula skeleton, i.e. `(<predicate> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_atomic_formula_skeleton;
/// # use pddl::types::{Variable, AtomicFormulaSkeleton, Predicate, Typed, Type, TypedList, ToTyped};
///
/// assert_eq!(parse_atomic_formula_skeleton("(at ?x - physob ?y - location)"), Ok(("",
///     AtomicFormulaSkeleton::new(
///         Predicate::from("at"),
///         TypedList::from_iter([
///             Variable::from("x").to_typed("physob"),
///             Variable::from("y").to_typed("location")
///         ]))
/// )));
/// ```
pub fn parse_atomic_formula_skeleton(input: &str) -> IResult<&str, AtomicFormulaSkeleton> {
    map(
        parens(tuple((parse_predicate, ws(typed_list(parse_variable))))),
        |tuple| AtomicFormulaSkeleton::from(tuple),
    )(input)
}
