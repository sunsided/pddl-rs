//! Provides parsers for constant definitions.

use nom::combinator::map;

use crate::parsers::{parse_name, prefix_expr, typed_list, ParseResult, Span};
use crate::types::Constants;

/// Parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_constants_def, preamble::*};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions, Constants, Name, ToTyped, TypedList};
/// let input = "(:constants B P D - physob)";
/// assert!(parse_constants_def(input).is_value(
///     Constants::new(TypedList::from_iter([
///         Name::from("B").to_typed("physob"),
///         Name::from("P").to_typed("physob"),
///         Name::from("D").to_typed("physob"),
///     ]))
/// ));
/// ```
pub fn parse_constants_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Constants> {
    map(prefix_expr(":constants", typed_list(parse_name)), |vec| {
        Constants::new(vec)
    })(input.into())
}

impl crate::parsers::Parser for Constants {
    type Item = Constants;

    /// See [`parse_constants_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_constants_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{Constants, Name, Parser, ToTyped, TypedList};

    #[test]
    fn test_parse() {
        let input = "(:constants B P D - physob)";
        assert!(
            Constants::parse(input).is_value(Constants::new(TypedList::from_iter([
                Name::from("B").to_typed("physob"),
                Name::from("P").to_typed("physob"),
                Name::from("D").to_typed("physob"),
            ])))
        );
    }
}
