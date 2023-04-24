//! Provides parsers for goal object declarations.

use crate::parsers::{parse_name, prefix_expr, typed_list};
use crate::types::Objects;
use nom::combinator::map;
use nom::IResult;

/// Parser for goal object declarations.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_objects_declaration};
/// # use pddl::types::{Name, Objects, ToTyped, Type};
///
/// let input = "(:objects train1 train2)";
/// assert_eq!(parse_problem_objects_declaration(input), Ok(("",
///     Objects::new([
///         Name::new("train1").to_typed(Type::OBJECT),
///         Name::new("train2").to_typed(Type::OBJECT),
///     ])
/// )));
/// ```
pub fn parse_problem_objects_declaration(input: &str) -> IResult<&str, Objects> {
    map(
        prefix_expr(":objects", typed_list(parse_name)),
        Objects::new,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for Objects<'a> {
    type Item = Objects<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_problem_objects_declaration(input)
    }
}
