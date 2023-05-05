//! Provides parsers for function types.

use crate::parsers::{parse_type, ParseResult, Span};
use crate::types::FunctionType;
use nom::combinator::map;

/// Parses a primitive type, i.e. `object | <name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_function_type, preamble::*};
/// # use pddl::{FunctionType};
/// # use pddl::Type;
/// assert!(parse_function_type("number").is_value(FunctionType::new(Type::Exactly("number".into()))));
/// assert!(parse_function_type("(either object number)").is_value(FunctionType::new(Type::from_iter(["object", "number"]))));
///```
pub fn parse_function_type<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FunctionType> {
    map(parse_type, FunctionType::from)(input.into())
}

impl crate::parsers::Parser for FunctionType {
    type Item = FunctionType;

    /// See [`parse_function_type`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_function_type(input)
    }
}
