//! Provides parsers for atomic function skeletons.

use crate::parsers::{parens, typed_list, ws};
use crate::parsers::{parse_function_symbol, parse_variable};
use crate::types::AtomicFunctionSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

/// Parser that parses an atomic function skeleton, i.e. `(<function-symbol> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_atomic_function_skeleton;
/// # use pddl::types::{Variable, AtomicFunctionSkeleton, Predicate, FunctionSymbol};
/// # use pddl::types::{ToTyped, TypedList};
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

impl<'a> crate::parsers::Parser<'a> for AtomicFunctionSkeleton<'a> {
    type Item = AtomicFunctionSkeleton<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_atomic_function_skeleton(input)
    }
}
