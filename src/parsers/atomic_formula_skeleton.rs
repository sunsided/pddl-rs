//! Provides parsers for atomic formula skeletons.

use crate::parsers::{parse_predicate, parse_variable, typed_list, ws};
use crate::types::AtomicFormulaSkeleton;
use nom::character::complete::char;
use nom::combinator::map;
use nom::sequence::{delimited, tuple};
use nom::IResult;

/// Parser that parses an atomic formula skeleton, i.e. `(<predicate> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_atomic_formula_skeleton;
/// # use pddl::types::{Variable, AtomicFormulaSkeleton, Predicate, Typed, Type};
///
/// assert_eq!(parse_atomic_formula_skeleton("(at ?x - physob ?y - location)"), Ok(("",
///     AtomicFormulaSkeleton::new(
///         Predicate::from("at"),
///         vec![
///             Typed::new(Variable::from("x"), Type::Exactly("physob".into())),
///             Typed::new(Variable::from("y"), Type::Exactly("location".into()))
///         ])
/// )));
/// ```
pub fn parse_atomic_formula_skeleton(input: &str) -> IResult<&str, AtomicFormulaSkeleton> {
    map(
        delimited(
            char('('),
            tuple((parse_predicate, ws(typed_list(parse_variable)))),
            char(')'),
        ),
        |tuple| AtomicFormulaSkeleton::from(tuple),
    )(input)
}
