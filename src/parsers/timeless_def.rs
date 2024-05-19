//! Provides parsers for timeless definitions.

use crate::parsers::{literal, parse_name, prefix_expr, space_separated_list1, ParseResult, Span};
use crate::types::Timeless;
use nom::combinator::map;

/// Parser for timeless definitions.
/// This is a PDDL 1.2 construct.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_timeless_def, preamble::*};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, Literal, Name, Objects, Timeless, ToTyped, Type};
/// let input = "(:timeless (= x y) (= a b))";
/// assert!(parse_timeless_def(input).is_value(
///     Timeless::from_iter([
///         Literal::AtomicFormula(
///             AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     "x".into(),
///                     "y".into()
///                 )
///             )
///         ),
///         Literal::AtomicFormula(
///             // ...
///             # AtomicFormula::Equality(
///             #     EqualityAtomicFormula::new(
///             #         "a".into(),
///             #         "b".into()
///             #     )
///             # )
///         )
///     ])
/// ));
/// ```
pub fn parse_timeless_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Timeless> {
    map(
        prefix_expr(":timeless", space_separated_list1(literal(parse_name))),
        Timeless::from_iter,
    )(input.into())
}

impl crate::parsers::Parser for Timeless {
    type Item = Timeless;

    /// See [`parse_timeless_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_timeless_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{AtomicFormula, EqualityAtomicFormula, Literal, Timeless};

    #[test]
    fn test_parse() {
        let input = "(:timeless (= x y) (= a b))";
        assert!(Timeless::parse(input).is_value(Timeless::from_iter([
            Literal::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                "x".into(),
                "y".into()
            ))),
            Literal::AtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                "a".into(),
                "b".into()
            )))
        ])));
    }
}
