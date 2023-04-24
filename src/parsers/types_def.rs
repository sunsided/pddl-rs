//! Provides parsers for constant definitions.

use crate::parsers::{parse_name, prefix_expr, typed_list};
use crate::types::Types;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_types_def;
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions};
/// # use pddl::{Name, Type, Typed, TypedList, Types};
/// let input = "(:types location physob)";
/// assert_eq!(parse_types_def(input), Ok(("",
///     Types::new(TypedList::from_iter([
///         Typed::new(Name::from("location"), Type::OBJECT),
///         Typed::new(Name::from("physob"), Type::OBJECT),
///     ]))
/// )));
/// ```
pub fn parse_types_def(input: &str) -> IResult<&str, Types> {
    map(prefix_expr(":types", typed_list(parse_name)), |vec| {
        Types::new(vec)
    })(input)
}

impl<'a> crate::parsers::Parser<'a> for Types<'a> {
    type Item = Types<'a>;

    /// See [`parse_types_def`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_types_def(input)
    }
}
