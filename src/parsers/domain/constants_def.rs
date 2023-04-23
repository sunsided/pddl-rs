//! Provides parsers for constant definitions.

use crate::parsers::domain::parse_name;
use crate::parsers::{prefix_expr, typed_list};
use crate::types::domain::Constants;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_constants_def;
/// # use pddl::types::domain::{Variable, AtomicFormulaSkeleton, Predicate, Typed, Type, PredicateDefinitions, Constants, Name, TypedList, ToTyped};
///
/// let input = "(:constants B P D - physob)";
/// assert_eq!(parse_constants_def(input), Ok(("",
///     Constants::new(TypedList::from_iter([
///         Name::from("B").to_typed("physob"),
///         Name::from("P").to_typed("physob"),
///         Name::from("D").to_typed("physob"),
///     ]))
/// )));
/// ```
pub fn parse_constants_def(input: &str) -> IResult<&str, Constants> {
    map(prefix_expr(":constants", typed_list(parse_name)), |vec| {
        Constants::new(vec)
    })(input)
}