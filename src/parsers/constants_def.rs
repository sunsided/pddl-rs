//! Provides parsers for constant definitions.

use crate::parsers::{parse_name, prefix_expr, typed_list};
use crate::types::Constants;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_constants_def;
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions, Constants, Name, ToTyped, TypedList};
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

impl<'a> crate::parsers::Parser<'a> for Constants<'a> {
    type Item = Constants<'a>;

    /// See [`parse_constants_def`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_constants_def(input)
    }
}
