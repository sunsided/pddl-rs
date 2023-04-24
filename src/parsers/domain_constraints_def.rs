//! Provides parsers for domain constraint definitions.

use crate::parsers::{parse_con_gd, prefix_expr};
use crate::types::DomainConstraintsDef;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses domain constraint definitions, i.e. `(:constraints <con-gd>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_domain_constraints_def, parse_functions_def};
/// # use pddl::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions, FunctionTypedList, FunctionTyped, AtomicFunctionSkeleton, FunctionSymbol, Functions, ConGD, DomainConstraintsDef};
/// # use pddl::{Type, Typed, TypedList};
///
/// let input = "(:constraints (and))";
/// assert_eq!(parse_domain_constraints_def(input), Ok(("",
///     DomainConstraintsDef::new(ConGD::new_and([]))
/// )));
/// ```
pub fn parse_domain_constraints_def(input: &str) -> IResult<&str, DomainConstraintsDef> {
    map(
        prefix_expr(":constraints", parse_con_gd),
        DomainConstraintsDef::new,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for DomainConstraintsDef<'a> {
    type Item = DomainConstraintsDef<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_domain_constraints_def(input)
    }
}
