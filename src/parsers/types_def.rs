//! Provides parsers for constant definitions.

use nom::combinator::map;

use crate::parsers::{parse_name, prefix_expr, typed_list, ParseResult, Span};
use crate::types::Types;

/// Parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_types_def, preamble::*};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions};
/// # use pddl::{Name, Type, Typed, TypedList, Types};
/// let input = "(:types location physob)";
/// assert!(parse_types_def(input).is_value(
///     Types::new(TypedList::from_iter([
///         Typed::new(Name::from("location"), Type::OBJECT),
///         Typed::new(Name::from("physob"), Type::OBJECT),
///     ]))
/// ));
/// ```
pub fn parse_types_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Types> {
    map(prefix_expr(":types", typed_list(parse_name)), |vec| {
        Types::new(vec)
    })(input.into())
}

impl crate::parsers::Parser for Types {
    type Item = Types;

    /// See [`parse_types_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_types_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Name, Parser, Type, Typed, TypedList, Types};

    #[test]
    fn test_parse() {
        let input = "(:types location physob)";
        assert!(
            Types::parse(input).is_value(Types::new(TypedList::from_iter([
                Typed::new(Name::from("location"), Type::OBJECT),
                Typed::new(Name::from("physob"), Type::OBJECT),
            ])))
        );
    }
}
