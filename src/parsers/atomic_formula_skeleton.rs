//! Provides parsers for atomic formula skeletons.

use crate::parsers::{parens, typed_list, ws, ParseResult, Span};
use crate::parsers::{parse_predicate, parse_variable};
use crate::types::AtomicFormulaSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;

/// Parser that parses an atomic formula skeleton, i.e. `(<predicate> <typed list (variable)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_atomic_formula_skeleton, Span, UnwrapValue};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate};
/// # use pddl::{ToTyped, TypedList};
/// assert!(parse_atomic_formula_skeleton(Span::new("(at ?x - physob ?y - location)")).is_value(
///     AtomicFormulaSkeleton::new(
///         Predicate::from("at"),
///         TypedList::from_iter([
///             Variable::from("x").to_typed("physob"),
///             Variable::from("y").to_typed("location")
///         ]))
/// ));
/// ```
pub fn parse_atomic_formula_skeleton<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, AtomicFormulaSkeleton<'a>> {
    map(
        parens(tuple((parse_predicate, ws(typed_list(parse_variable))))),
        |tuple| AtomicFormulaSkeleton::from(tuple),
    )(input.into())
}

impl<'a> crate::parsers::Parser<'a> for AtomicFormulaSkeleton<'a> {
    type Item = AtomicFormulaSkeleton<'a>;

    /// See [`parse_atomic_formula_skeleton`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_atomic_formula_skeleton(input.into())
    }
}
