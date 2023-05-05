//! Provides parsers for timed goal definitions.

use crate::parsers::{parse_gd, parse_interval, parse_time_specifier};
use crate::parsers::{prefix_expr, ParseResult, Span};
use crate::types::TimedGD;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parser for timed goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_timed_gd, preamble::*};
/// # use pddl::{AtomicFormula, GoalDefinition, Interval, Term, TimedGD, TimeSpecifier};
/// assert!(parse_timed_gd("(at start (= x y))").is_value(
///     TimedGD::new_at(
///         TimeSpecifier::Start,
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// ));
///
/// assert!(parse_timed_gd("(over all (= x y))").is_value(
///     TimedGD::new_over(
///         Interval::All,
///         GoalDefinition::AtomicFormula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// ));
/// ```
pub fn parse_timed_gd<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, TimedGD<'a>> {
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

    alt((at, over))(input.into())
}

impl<'a> crate::parsers::Parser<'a> for TimedGD<'a> {
    type Item = TimedGD<'a>;

    /// See [`parse_timed_gd`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_timed_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "(over all (can-move ?from-waypoint ?to-waypoint))";
        let (_, _gd) = parse_timed_gd(Span::new(input)).unwrap();
    }
}
