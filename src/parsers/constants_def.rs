//! Provides parsers for constant definitions.

use crate::parsers::{parse_name, prefix_expr, typed_list};
use crate::types::Constants;
use nom::combinator::map;
use nom::IResult;

/// Parser that parses constant definitions, i.e. `(:constants <typed list (name)>)`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_constants_def;
/// # use pddl::types::{Variable, AtomicFormulaSkeleton, Predicate, Typed, Type, PredicateDefinitions, Constants, Name};
///
/// let input = "(:constants B P D - physob)";
/// assert_eq!(parse_constants_def(input), Ok(("",
///     Constants::new(vec![
///         Typed::new(Name::from("B"), Type::Exactly("physob".into())),
///         Typed::new(Name::from("P"), Type::Exactly("physob".into())),
///         Typed::new(Name::from("D"), Type::Exactly("physob".into())),
///     ])
/// )));
/// ```
pub fn parse_constants_def(input: &str) -> IResult<&str, Constants> {
    map(prefix_expr(":constants", typed_list(parse_name)), |vec| {
        Constants::new(vec)
    })(input)
}
