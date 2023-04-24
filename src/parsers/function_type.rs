//! Provides parsers for function types.

use crate::parsers::parse_type;
use crate::types::FunctionType;
use nom::combinator::map;
use nom::IResult;

/// Parses a primitive type, i.e. `object | <name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_function_type;
/// # use pddl::{FunctionType};
/// # use pddl::Type;
/// assert_eq!(parse_function_type("number"), Ok(("", FunctionType::new(Type::Exactly("number".into())))));
/// assert_eq!(parse_function_type("(either object number)"), Ok(("", FunctionType::new(Type::from_iter(["object", "number"])))));
///```
pub fn parse_function_type(input: &str) -> IResult<&str, FunctionType> {
    map(parse_type, FunctionType::from)(input)
}

impl<'a> crate::parsers::Parser<'a> for FunctionType<'a> {
    type Item = FunctionType<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_function_type(input)
    }
}
