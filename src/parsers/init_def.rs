//! Provides parsers for goal initial state definitions.

use crate::parsers::{parse_init_el, prefix_expr, space_separated_list0, ParseResult, Span};
use crate::types::InitElements;
use nom::combinator::map;

/// Parser for goal initial state definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_problem_init_def, preamble::*};
/// # use pddl::{AtomicFormula, InitElement, InitElements, NameLiteral, Number};
/// let input = "(:init (train-not-in-use train1) (at 10 (train-not-in-use train2)))";
/// assert!(parse_problem_init_def(input).is_value(
///     InitElements::from_iter([
///         InitElement::new_literal(
///             NameLiteral::new(
///                 AtomicFormula::new_predicate(
///                     "train-not-in-use".into(),
///                     ["train1".into()]
///                 )
///             )
///         ),
///         InitElement::new_at(
///             Number::from(10),
///             NameLiteral::new(
///                 AtomicFormula::new_predicate(
///                     "train-not-in-use".into(),
///                     ["train2".into()]
///                 )
///             )
///         )
///     ])
/// ));
/// ```
pub fn parse_problem_init_def<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, InitElements<'a>> {
    map(
        prefix_expr(":init", space_separated_list0(parse_init_el)),
        InitElements::new,
    )(input.into())
}
impl<'a> crate::parsers::Parser<'a> for InitElements<'a> {
    type Item = InitElements<'a>;

    /// See [`parse_problem_init_def`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_problem_init_def(input.into())
    }
}
