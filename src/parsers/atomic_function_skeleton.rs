//! Provides parsers for atomic function skeletons.

use crate::parsers::{parens, typed_list, ws, ParseResult, Span};
use crate::parsers::{parse_function_symbol, parse_variable};
use crate::types::AtomicFunctionSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;

/// Parser that parses an atomic function skeleton, i.e. `(<function-symbol> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_atomic_function_skeleton, Span, UnwrapValue};
/// # use pddl::{Variable, AtomicFunctionSkeleton, Predicate, FunctionSymbol};
/// # use pddl::{ToTyped, TypedList};
/// assert!(parse_atomic_function_skeleton(Span::new("(battery-amount ?r - rover)")).is_value(
///     AtomicFunctionSkeleton::new(
///         FunctionSymbol::from("battery-amount"),
///         TypedList::from_iter([
///             Variable::from("r").to_typed("rover")
///         ]))
/// ));
/// ```
pub fn parse_atomic_function_skeleton(input: Span) -> ParseResult<AtomicFunctionSkeleton> {
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

    /// See [`parse_atomic_function_skeleton`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_atomic_function_skeleton(input)
    }
}
