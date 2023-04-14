//! Provides parsers for primitive types.

use crate::parsers::name::parse_name;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::recognize;
use nom::IResult;

/// Parses a primitive type, i.e. `object | <name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_primitive_type;
/// assert_eq!(parse_primitive_type("object"), Ok(("", "object")));
/// assert_eq!(parse_primitive_type("number"), Ok(("", "number")));
/// assert_eq!(parse_primitive_type("a-1_2"), Ok(("", "a-1_2")));
/// assert_eq!(parse_primitive_type("obj!ect"), Ok(("!ect", "obj")));
///```
pub fn parse_primitive_type(input: &str) -> IResult<&str, &str> {
    recognize(alt((tag("object"), parse_name)))(input)
}
