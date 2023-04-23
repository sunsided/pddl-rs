//! Provides parsers for constant definitions.

use crate::parsers::domain::{function_typed_list, parse_atomic_function_skeleton};
use crate::parsers::prefix_expr;
use crate::types::domain::Functions;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_functions_def;
/// # use pddl::types::domain::{Variable, AtomicFormulaSkeleton, Predicate, Typed, Type, PredicateDefinitions, Types, Name, TypedList, FunctionTypedList, FunctionTyped, AtomicFunctionSkeleton, FunctionSymbol, Functions};
///
/// let input = "(:functions (battery-amount ?r - rover))";
/// assert_eq!(parse_functions_def(input), Ok(("",
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
/// )));
/// ```
pub fn parse_functions_def(input: &str) -> IResult<&str, Functions> {
    map(
        prefix_expr(
            ":functions",
            function_typed_list(parse_atomic_function_skeleton),
        ),
        |vec| Functions::new(vec),
    )(input)
}
