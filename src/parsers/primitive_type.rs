//! Provides parsers for primitive types.

use crate::parsers::name::parse_name;
use crate::types::PrimitiveType;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::recognize;
use nom::IResult;

/// Parses a primitive type, i.e. `object | <name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_primitive_type;
/// assert_eq!(parse_primitive_type("object"), Ok(("", "object".into())));
/// assert_eq!(parse_primitive_type("number"), Ok(("", "number".into())));
/// assert_eq!(parse_primitive_type("a-1_2"), Ok(("", "a-1_2".into())));
/// assert_eq!(parse_primitive_type("obj!ect"), Ok(("!ect", "obj".into())));
///```
pub fn parse_primitive_type(input: &str) -> IResult<&str, PrimitiveType> {
    let (remaining, r#type) = recognize(alt((tag("object"), parse_name)))(input)?;
    Ok((remaining, PrimitiveType::from(r#type)))
}
