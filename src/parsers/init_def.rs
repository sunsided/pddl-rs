//! Provides parsers for goal initial state definitions.

use crate::parsers::{parse_init_el, prefix_expr, space_separated_list0};
use crate::types::InitElements;
use nom::combinator::map;
use nom::IResult;

/// Parser for goal initial state definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_goal_init_def};
/// # use pddl::types::{AtomicFormula, InitElement, InitElements, NameLiteral, Number};
///
/// let input = "(:init (train-not-in-use train1) (at 10 (train-not-in-use train2)))";
/// assert_eq!(parse_goal_init_def(input), Ok(("",
///     InitElements::new([
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
/// )));
/// ```
pub fn parse_goal_init_def(input: &str) -> IResult<&str, InitElements> {
    map(
        prefix_expr(":init", space_separated_list0(parse_init_el)),
        InitElements::new,
    )(input)
}
