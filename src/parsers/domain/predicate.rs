//! Provides parsers for predicates.

use crate::parsers::domain::name::parse_name;
use crate::types::domain::Predicate;
use nom::combinator::map;
use nom::IResult;

/// Parses a predicate, i.e. `<name>`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_predicate;
/// assert_eq!(parse_predicate("abcde"), Ok(("", "abcde".into())));
/// assert_eq!(parse_predicate("a-1_2"), Ok(("", "a-1_2".into())));
/// assert_eq!(parse_predicate("Z01"), Ok(("", "Z01".into())));
/// assert_eq!(parse_predicate("x-_-_"), Ok(("", "x-_-_".into())));
///
/// assert!(parse_predicate("").is_err());
/// assert!(parse_predicate(".").is_err());
/// assert!(parse_predicate("-abc").is_err());
/// assert!(parse_predicate("0124").is_err());
/// assert!(parse_predicate("-1").is_err());
///```
pub fn parse_predicate(input: &str) -> IResult<&str, Predicate> {
    map(parse_name, Predicate::from)(input)
}
