//! Provides parsers for domain constraint definitions.

use nom::combinator::map;

use crate::parsers::{parse_con_gd, prefix_expr, ParseResult, Span};
use crate::types::DomainConstraintsDef;

/// Parses domain constraint definitions, i.e. `(:constraints <con-gd>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_domain_constraints_def, parse_functions_def, preamble::*};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions, FunctionTypedList, FunctionTyped, AtomicFunctionSkeleton, FunctionSymbol, Functions, ConGD, DomainConstraintsDef};
/// # use pddl::{Type, Typed, TypedList};
///
/// let input = "(:constraints (and))";
/// assert!(parse_domain_constraints_def(input).is_value(
///     DomainConstraintsDef::new(ConGD::new_and([]))
/// ));
/// ```
pub fn parse_domain_constraints_def<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, DomainConstraintsDef> {
    map(
        prefix_expr(":constraints", parse_con_gd),
        DomainConstraintsDef::new,
    )(input.into())
}

impl crate::parsers::Parser for DomainConstraintsDef {
    type Item = DomainConstraintsDef;

    /// See [`parse_domain_constraints_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_domain_constraints_def(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{ConGD, DomainConstraintsDef, Parser};

    #[test]
    fn test_parse() {
        let input = "(:constraints (and))";
        assert!(DomainConstraintsDef::parse(input)
            .is_value(DomainConstraintsDef::new(ConGD::new_and([]))));
    }
}
