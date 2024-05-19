//! Provides parsers for types.

use crate::parsers::{parse_primitive_type, ParseResult, Span};
use crate::parsers::{prefix_expr, space_separated_list1};
use crate::types::{PrimitiveType, Type};
use nom::error::ErrorKind;
use nom::error_position;

/// Parses a primitive type, i.e. `object | <name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_type, preamble::*};
/// # use pddl::Type;
/// assert!(parse_type(Span::new("object")).is_value(Type::Exactly("object".into())));
/// assert!(parse_type(Span::new("(either object number)")).is_value(Type::from_iter(["object", "number"])));
///```
pub fn parse_type<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Type> {
    let input = input.into();

    if let Ok((remaining, r#type)) = parse_primitive_type(input) {
        return Ok((remaining, Type::Exactly(r#type)));
    }

    if let Ok((remaining, types)) = parse_either_type(input) {
        return Ok((remaining, Type::EitherOf(types)));
    }

    Err(nom::Err::Failure(error_position!(
        input,
        ErrorKind::Alt
    )))
}

/// Parses a either type, i.e. `(either a b c)`.
fn parse_either_type<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Vec<PrimitiveType>> {
    prefix_expr("either", space_separated_list1(parse_primitive_type))(input.into())
}

impl crate::parsers::Parser for Type {
    type Item = Type;

    /// See [`parse_type`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_type(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::Match;

    #[test]
    fn explicit_works() {
        assert!(parse_type(Span::new("object")).is_exactly(Type::from("object")));
    }

    #[test]
    fn either_works() {
        assert!(parse_type(Span::new("(either object number)"))
            .is_exactly(Type::from_iter(["object", "number"])));
    }

    #[test]
    fn either_specific_works() {
        assert!(
            parse_either_type(Span::new("(either object number)")).is_exactly(vec![
                PrimitiveType::from("object"),
                PrimitiveType::from("number")
            ])
        );
    }
}
