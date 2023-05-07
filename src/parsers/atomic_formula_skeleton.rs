//! Provides parsers for atomic formula skeletons.

use crate::parsers::{parens, typed_list, ws, ParseResult, Span};
use crate::parsers::{parse_predicate, parse_variable};
use crate::types::AtomicFormulaSkeleton;
use nom::combinator::map;
use nom::sequence::tuple;

/// Parses an atomic formula skeleton, i.e. `(<predicate> <typed list (variable)>)`.
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
) -> ParseResult<'a, AtomicFormulaSkeleton> {
    map(
        parens(tuple((parse_predicate, ws(typed_list(parse_variable))))),
        |tuple| AtomicFormulaSkeleton::from(tuple),
    )(input.into())
}

impl crate::parsers::Parser for AtomicFormulaSkeleton {
    type Item = AtomicFormulaSkeleton;

    /// Parses an atomic formula skeleton.
    ///
    /// ## Example
    /// ```
    /// # use pddl::parsers::{parse_atomic_formula_skeleton, Span, UnwrapValue};
    /// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, Parser};
    /// # use pddl::{ToTyped, TypedList};
    /// let (_, value) = AtomicFormulaSkeleton::parse("(at ?x - physob ?y - location)").unwrap();
    ///
    /// assert_eq!(value,
    ///     AtomicFormulaSkeleton::new(
    ///         Predicate::from("at"),
    ///         TypedList::from_iter([
    ///             Variable::from("x").to_typed("physob"),
    ///             Variable::from("y").to_typed("location")
    ///         ]))
    /// );
    /// ```
    ///
    /// ## See also
    /// See [`parse_atomic_formula_skeleton`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_atomic_formula_skeleton(input)
    }
}
