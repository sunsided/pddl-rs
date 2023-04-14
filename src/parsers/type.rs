//! Provides parsers for types.

use crate::parsers::{definition_section, parse_primitive_type, space_separated_list1, ws};
use crate::types::PrimitiveType;
use crate::Type;
use nom::branch::alt;
use nom::combinator::map_parser;
use nom::IResult;

/// Parses a primitive type, i.e. `object | <name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_type;
/// # use pddl::Type;
/// assert_eq!(parse_type("object"), Ok(("", Type::Exactly("object".into()))));
/// assert_eq!(parse_type("(either object number)"), Ok(("", Type::from_iter(["object", "number"]))));
///```
pub fn parse_type(input: &str) -> IResult<&str, Type> {
    let single = ws(map_parser(parse_primitive_type, multiply));

    let (remaining, types) = alt((single, parse_either_type))(input)?;

    if types.len() == 1 {
        Ok((remaining, Type::Exactly(types.into_iter().next().unwrap())))
    } else {
        Ok((
            remaining,
            Type::EitherOf(types.into_iter().map(PrimitiveType::from).collect()),
        ))
    }
}

/// Helper parser that maps a single [`PrimitiveType`] into a vector.
fn multiply<'a>(
    input: PrimitiveType<'a>,
) -> Result<(PrimitiveType<'a>, Vec<PrimitiveType<'a>>), nom::Err<nom::error::Error<&str>>> {
    Ok((PrimitiveType::default(), vec![input]))
}

/// Parses a either type, i.e. `(either a b c)`.
fn parse_either_type(input: &str) -> IResult<&str, Vec<PrimitiveType>> {
    definition_section("either", space_separated_list1(parse_primitive_type))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_works() {
        assert_eq!(parse_type("object"), Ok(("", Type::from("object"))));
    }

    #[test]
    fn either_works() {
        assert_eq!(
            parse_type("(either object number)"),
            Ok(("", Type::from_iter(["object", "number"])))
        );
    }

    #[test]
    fn either_specific_works() {
        assert_eq!(
            parse_either_type("(either object number)"),
            Ok((
                "",
                vec![PrimitiveType::from("object"), PrimitiveType::from("number")]
            ))
        );
    }
}
