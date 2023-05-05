//! Provides parsers for constant definitions.

use crate::parsers::{function_typed_list, parse_atomic_function_skeleton};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::Functions;
use nom::combinator::map;

/// Parser that parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_functions_def, preamble::*};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions, FunctionTypedList, FunctionTyped, AtomicFunctionSkeleton, FunctionSymbol, Functions};
/// # use pddl::{Type, Typed, TypedList};
/// let input = "(:functions (battery-amount ?r - rover))";
/// assert!(parse_functions_def(input).is_value(
///     Functions::from_iter([
///         FunctionTyped::new_number(
///             AtomicFunctionSkeleton::new(
///                 FunctionSymbol::from_str("battery-amount"),
///                 TypedList::from_iter([
///                     Typed::new(Variable::from("r"), Type::Exactly("rover".into()))
///                 ])
///             )
///         )
///     ])
/// ));
/// ```
pub fn parse_functions_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Functions<'a>> {
    map(
        prefix_expr(
            ":functions",
            function_typed_list(parse_atomic_function_skeleton),
        ),
        |vec| Functions::new(vec),
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for Functions<'a> {
    type Item = Functions<'a>;

    /// See [`parse_functions_def`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_functions_def(input.into())
    }
}
