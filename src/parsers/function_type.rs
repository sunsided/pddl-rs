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
/// assert!(parse_function_type("number".into()).is_value(FunctionType::new(Type::Exactly("number".into()))));
/// assert!(parse_function_type("(either object number)".into()).is_value(FunctionType::new(Type::from_iter(["object", "number"]))));
///```
pub fn parse_function_type(input: Span) -> ParseResult<FunctionType> {
    map(parse_type, FunctionType::from)(input)
}

impl<'a> crate::parsers::Parser<'a> for FunctionType<'a> {
    type Item = FunctionType<'a>;

    /// See [`parse_function_type`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_function_type(input)
    }
}
