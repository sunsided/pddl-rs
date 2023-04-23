//! Provides parsers for types.

use crate::parsers::domain::parse_primitive_type;
use crate::parsers::utility::{prefix_expr, space_separated_list1};
use crate::types::utility::{PrimitiveType, Type};
use nom::error::ErrorKind;
use nom::{error_position, IResult};

/// Parses a primitive type, i.e. `object | <name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::utility::parse_type;
/// # use pddl::types::utility::Type;
/// assert_eq!(parse_type("object"), Ok(("", Type::Exactly("object".into()))));
/// assert_eq!(parse_type("(either object number)"), Ok(("", Type::from_iter(["object", "number"]))));
///```
pub fn parse_type(input: &str) -> IResult<&str, Type> {
    if let Ok((remaining, r#type)) = parse_primitive_type(input) {
        return Ok((remaining, Type::Exactly(r#type)));
    }

    if let Ok((remaining, types)) = parse_either_type(input) {
        return Ok((remaining, Type::EitherOf(types)));
    }

    Err(nom::Err::Failure(error_position!(input, ErrorKind::Alt)))
}

/// Parses a either type, i.e. `(either a b c)`.
fn parse_either_type(input: &str) -> IResult<&str, Vec<PrimitiveType>> {
    prefix_expr("either", space_separated_list1(parse_primitive_type))(input)
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
