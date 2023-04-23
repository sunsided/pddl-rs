//! Provides parsers for domain constraint definitions.

use crate::parsers::{parse_con_gd, prefix_expr};
use crate::types::ConGD;
use nom::IResult;

/// Parser that parses domain constraint definitions, i.e. `(:constraints <con-gd>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_domain_constraints_def, parse_functions_def};
/// # use pddl::types::{Variable, AtomicFormulaSkeleton, Predicate, PredicateDefinitions, FunctionTypedList, FunctionTyped, AtomicFunctionSkeleton, FunctionSymbol, Functions, ConGD};
/// # use pddl::types::{Type, Typed, TypedList};
///
/// let input = "(:constraints (and))";
/// assert_eq!(parse_domain_constraints_def(input), Ok(("",
///     ConGD::new_and([])
/// )));
/// ```
pub fn parse_domain_constraints_def(input: &str) -> IResult<&str, ConGD> {
    prefix_expr(":constraints", parse_con_gd)(input)
}
