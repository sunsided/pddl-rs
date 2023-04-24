//! Provides parsers for timeless definitions.

use crate::parsers::{literal, parse_name, prefix_expr, space_separated_list1};
use crate::types::Timeless;
use nom::combinator::map;
use nom::IResult;

/// Parser for timeless definitions.
/// This is a PDDL 1.2 construct.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_timeless_def;
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, Literal, Name, Objects, Timeless, ToTyped, Type};
/// let input = "(:timeless (= x y) (= a b))";
/// assert_eq!(parse_timeless_def(input), Ok(("",
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
/// )));
/// ```
pub fn parse_timeless_def(input: &str) -> IResult<&str, Timeless> {
    map(
        prefix_expr(":timeless", space_separated_list1(literal(parse_name))),
        Timeless::from_iter,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for Timeless<'a> {
    type Item = Timeless<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_timeless_def(input)
    }
}
