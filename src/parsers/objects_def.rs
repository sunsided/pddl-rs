//! Provides parsers for goal object declarations.

use crate::parsers::{parse_name, prefix_expr, typed_list, ParseResult, Span};
use crate::types::Objects;
use nom::combinator::map;

/// Parser for goal object declarations.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_objects_declaration, preamble::*};
/// # use pddl::{Name, Objects, ToTyped, Type};
/// let input = "(:objects train1 train2)";
/// assert!(parse_problem_objects_declaration(input.into()).is_value(
///     Objects::new([
///         Name::new("train1").to_typed(Type::OBJECT),
///         Name::new("train2").to_typed(Type::OBJECT),
///     ])
/// ));
/// ```
pub fn parse_problem_objects_declaration(input: Span) -> ParseResult<Objects> {
    map(
        prefix_expr(":objects", typed_list(parse_name)),
        Objects::new,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for Objects<'a> {
    type Item = Objects<'a>;

    /// See [`parse_problem_objects_declaration`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_objects_declaration(input.into())
    }
}
