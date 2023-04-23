//! Provides parsers for atomic function skeletons.

use crate::parsers::domain::{parse_function_symbol, parse_variable};
use crate::parsers::utility::{parens, typed_list, ws};
use crate::types::domain::AtomicFunctionSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

/// Parser that parses an atomic function skeleton, i.e. `(<function-symbol> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_atomic_function_skeleton;
/// # use pddl::types::domain::{Variable, AtomicFunctionSkeleton, Predicate, FunctionSymbol};
/// # use pddl::types::utility::{ToTyped, TypedList};
///
/// assert_eq!(parse_atomic_function_skeleton("(battery-amount ?r - rover)"), Ok(("",
///     AtomicFunctionSkeleton::new(
///         FunctionSymbol::from("battery-amount"),
///         TypedList::from_iter([
///             Variable::from("r").to_typed("rover")
///         ]))
/// )));
/// ```
pub fn parse_atomic_function_skeleton(input: &str) -> IResult<&str, AtomicFunctionSkeleton> {
    map(
        parens(tuple((
            parse_function_symbol,
            ws(typed_list(parse_variable)),
        ))),
        |tuple| AtomicFunctionSkeleton::from(tuple),
    )(input)
}
