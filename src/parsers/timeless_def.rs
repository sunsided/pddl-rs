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
/// assert!(parse_timeless_def(input.into()).is_value(
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
pub fn parse_timeless_def(input: Span) -> ParseResult<Timeless> {
    map(
        prefix_expr(":timeless", space_separated_list1(literal(parse_name))),
        Timeless::from_iter,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for Timeless<'a> {
    type Item = Timeless<'a>;

    /// See [`parse_timeless_def`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_timeless_def(input)
    }
}
