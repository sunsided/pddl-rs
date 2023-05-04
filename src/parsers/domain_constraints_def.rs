//! Provides parsers for domain constraint definitions.

use crate::parsers::{parse_con_gd, prefix_expr, ParseResult, Span};
use crate::types::DomainConstraintsDef;
use nom::combinator::map;

/// Parser that parses domain constraint definitions, i.e. `(:constraints <con-gd>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_domain_constraints_def, parse_functions_def, preamble::*};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions, FunctionTypedList, FunctionTyped, AtomicFunctionSkeleton, FunctionSymbol, Functions, ConGD, DomainConstraintsDef};
/// # use pddl::{Type, Typed, TypedList};
///
/// let input = "(:constraints (and))";
/// assert!(parse_domain_constraints_def(input.into()).is_value(
///     DomainConstraintsDef::new(ConGD::new_and([]))
/// ));
/// ```
pub fn parse_domain_constraints_def(input: Span) -> ParseResult<DomainConstraintsDef> {
    map(
        prefix_expr(":constraints", parse_con_gd),
        DomainConstraintsDef::new,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for DomainConstraintsDef<'a> {
    type Item = DomainConstraintsDef<'a>;

    /// See [`parse_domain_constraints_def`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_domain_constraints_def(input.into())
    }
}
