//! Provides parsers for timed goal definitions.

use crate::parsers::prefix_expr;
use crate::parsers::{parse_gd, parse_interval, parse_time_specifier};
use crate::types::TimedGD;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser for timed goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_timed_gd};
/// # use pddl::types::{AtomicFormula, GoalDefinition, Interval, Term, TimedGD, TimeSpecifier};
///
/// assert_eq!(parse_timed_gd("(at start (= x y))"), Ok(("",
///     TimedGD::new_at(
///         TimeSpecifier::Start,
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_timed_gd("(over all (= x y))"), Ok(("",
///     TimedGD::new_over(
///         Interval::All,
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// )));
/// ```
pub fn parse_timed_gd(input: &str) -> IResult<&str, TimedGD> {
    let at = map(
        prefix_expr(
            "at",
            tuple((parse_time_specifier, preceded(multispace1, parse_gd))),
        ),
        TimedGD::from,
    );

    let over = map(
        prefix_expr(
            "over",
            tuple((parse_interval, preceded(multispace1, parse_gd))),
        ),
        TimedGD::from,
    );

    alt((at, over))(input)
}

#[cfg_attr(docsrs, doc(cfg(feature = "parser")))]
#[cfg(feature = "parser")]
impl<'a> crate::parsers::Parser<'a> for TimedGD<'a> {
    type Item = TimedGD<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_timed_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "(over all (can-move ?from-waypoint ?to-waypoint))";
        let (_, _gd) = parse_timed_gd(input).unwrap();
    }
}
