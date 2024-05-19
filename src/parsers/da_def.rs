//! Provides parsers for durative action definitions.

use crate::parsers::{empty_or, parens, prefix_expr, typed_list, ParseResult, Span};
use crate::parsers::{
    parse_da_effect, parse_da_gd, parse_da_symbol, parse_duration_constraint, parse_variable,
};
use crate::types::DurativeActionDefinition;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses a durative action definition.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_action_def, parse_da_def, preamble::*};
/// # use pddl::{ActionDefinition, ActionSymbol, AtomicFormula, CEffect, Effects, GoalDefinition, Literal, PEffect, Predicate, Preference, PreferenceGD, PreconditionGoalDefinition, Term, Variable};
/// let input = r#"(:durative-action move
///         :parameters
///             (?r - rover
///              ?fromwp - waypoint
///              ?towp - waypoint)
///
///         :duration
///             (= ?duration 5)
///
///         :condition
///             (and
///                 (at start (rover ?rover))
///                 (at start (waypoint ?from-waypoint))
///                 (at start (waypoint ?to-waypoint))
///                 (over all (can-move ?from-waypoint ?to-waypoint))
///                 (at start (at ?rover ?from-waypoint))
///                 (at start (> (battery-amount ?rover) 8)))
///
///         :effect
///             (and
///                 (decrease (fuel-level ?t) (* 2 #t))
///                 (at end (at ?rover ?to-waypoint))
///                 (at end (been-at ?rover ?to-waypoint))
///                 (at start (not (at ?rover ?from-waypoint)))
///                 (at start (decrease (battery-amount ?rover) 8))
///                 (at end (increase (distance-travelled) 5))
///                 )
///     )"#;
///
/// let (_, da_def) = parse_da_def(input).unwrap();
/// assert_eq!(da_def.symbol(), &"move".into());
/// assert_eq!(da_def.parameters().len(), 3);
/// assert!(da_def.duration().is_some());
/// assert!(da_def.condition().is_some());
/// assert!(da_def.effect().is_some());
/// ```
pub fn parse_da_def<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, DurativeActionDefinition> {
    let parameters = preceded(
        tag(":parameters"),
        preceded(multispace1, parens(typed_list(parse_variable))),
    );

    let duration = preceded(
        tag(":duration"),
        preceded(multispace1, parse_duration_constraint),
    );
    let condition = preceded(
        tag(":condition"),
        preceded(multispace1, empty_or(parse_da_gd)),
    );
    let effect = preceded(
        tag(":effect"),
        preceded(multispace1, empty_or(parse_da_effect)),
    );

    let da_def = prefix_expr(
        ":durative-action",
        tuple((
            parse_da_symbol,
            preceded(multispace1, parameters),
            // <da-def body>
            preceded(multispace1, duration),
            preceded(multispace1, condition),
            preceded(multispace1, effect),
        )),
    );

    map(
        da_def,
        |(symbol, parameters, duration, condition, effect)| {
            DurativeActionDefinition::new(symbol, parameters, duration, condition, effect)
        },
    )(input.into())
}

impl crate::parsers::Parser for DurativeActionDefinition {
    type Item = DurativeActionDefinition;

    /// See [`parse_da_def`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_da_def(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn it_works() {
        let input = r#"(:durative-action move
                :parameters
                    (?r - rover
                     ?fromwp - waypoint
                     ?towp - waypoint)

                :duration
                    (= ?duration 5)

                :condition
                    (and
                        (at start (rover ?rover))
                        (at start (waypoint ?from-waypoint))
                        (at start (waypoint ?to-waypoint))
                        (over all (can-move ?from-waypoint ?to-waypoint))
                        (at start (at ?rover ?from-waypoint))
                        (at start (> (battery-amount ?rover) 8)))

                :effect
                    (and
                        (decrease (fuel-level ?t) (* 2 #t))
                        (at end (at ?rover ?to-waypoint))
                        (at end (been-at ?rover ?to-waypoint))
                        (at start (not (at ?rover ?from-waypoint)))
                        (at start (decrease (battery-amount ?rover) 8))
                        (at end (increase (distance-travelled) 5))
                        )
            )"#;
        let (_, _gd) = DurativeActionDefinition::parse(Span::new(input)).unwrap();
    }
}
