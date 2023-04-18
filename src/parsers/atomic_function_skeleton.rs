//! Provides parsers for atomic function skeletons.

use crate::parsers::{parens, parse_function_symbol, parse_variable, typed_list, ws};
use crate::types::AtomicFunctionSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

/// Parser that parses an atomic function skeleton, i.e. `(<function-symbol> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_atomic_function_skeleton;
/// # use pddl::types::{Variable, AtomicFunctionSkeleton, Predicate, Typed, Type, FunctionSymbol, TypedList};
///
/// assert_eq!(parse_atomic_function_skeleton("(battery-amount ?r - rover)"), Ok(("",
///     AtomicFunctionSkeleton::new(
///         FunctionSymbol::from("battery-amount"),
///         TypedList::from_iter([
///             Typed::new(Variable::from("r"), Type::Exactly("rover".into()))
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
