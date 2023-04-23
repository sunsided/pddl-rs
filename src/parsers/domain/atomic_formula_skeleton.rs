//! Provides parsers for atomic formula skeletons.

use crate::parsers::domain::{parse_predicate, parse_variable};
use crate::parsers::utility::{parens, typed_list, ws};
use crate::types::domain::AtomicFormulaSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

/// Parser that parses an atomic formula skeleton, i.e. `(<predicate> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_atomic_formula_skeleton;
/// # use pddl::types::domain::{Variable, AtomicFormulaSkeleton, Predicate};
/// # use pddl::types::utility::{ToTyped, TypedList};
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
