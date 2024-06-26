//! Provides parsers for durative action goal definitions.

use crate::parsers::{parens, prefix_expr, space_separated_list0, typed_list, ParseResult, Span};
use crate::parsers::{parse_pref_timed_gd, parse_variable};
use crate::types::DurativeActionGoalDefinition;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_da_gd, preamble::*};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Preference, PreferenceName, PreferenceGD, Term, Variable, DurativeActionGoalDefinition, PrefTimedGD, TimedGD, TimeSpecifier, Interval};
/// # use pddl::{Typed, TypedList};
/// assert!(parse_da_gd("(at start (= x y))").is_value(
///     DurativeActionGoalDefinition::Timed(
///         PrefTimedGD::Required(
///             TimedGD::new_at(
///                 TimeSpecifier::Start,
///                 GoalDefinition::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("x".into()),
///                         Term::Name("y".into())
///                     )
///                 )
///             )
///         )
///     )
/// ));
///
/// assert!(parse_da_gd("(and )").is_value(
///     DurativeActionGoalDefinition::new_and([])
/// ));
///
/// assert!(parse_da_gd("(and (at start (= x y)) (over all (= a b)))").is_value(
///     DurativeActionGoalDefinition::new_and([
///         DurativeActionGoalDefinition::Timed(PrefTimedGD::Required(
///             TimedGD::new_at(
///                 TimeSpecifier::Start,
///                 GoalDefinition::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("x".into()),
///                         Term::Name("y".into())
///                     )
///                 )
///             )
///         )),
///         DurativeActionGoalDefinition::Timed(PrefTimedGD::Required(
///             TimedGD::new_over(
///                 Interval::All,
///                 GoalDefinition::AtomicFormula(
///                     AtomicFormula::new_equality(
///                         Term::Name("a".into()),
///                         Term::Name("b".into())
///                     )
///                 )
///             )
///         ))
///     ])
/// ));
///
/// assert!(parse_da_gd("(forall (?a ?b) (at start (= a b)))").is_value(
///     DurativeActionGoalDefinition::new_forall(
///         TypedList::from_iter([
///             Typed::new_object(Variable::from_str("a")),
///             Typed::new_object(Variable::from_str("b")),
///         ]),
///         DurativeActionGoalDefinition::Timed(
///             PrefTimedGD::Required(
///                 TimedGD::new_at(
///                     TimeSpecifier::Start,
///                     GoalDefinition::AtomicFormula(
///                         AtomicFormula::new_equality(
///                             Term::Name("a".into()),
///                             Term::Name("b".into())
///                         )
///                     )
///                 )
///             )
///         )
///     )
/// ));
/// ```
pub fn parse_da_gd<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, DurativeActionGoalDefinition> {
    let pref_timed_gd = map(parse_pref_timed_gd, DurativeActionGoalDefinition::new_timed);
    let and = map(
        prefix_expr("and", space_separated_list0(parse_da_gd)),
        DurativeActionGoalDefinition::new_and,
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_da_gd),
            )),
        ),
        |(vars, gd)| DurativeActionGoalDefinition::new_forall(vars, gd),
    );

    alt((forall, and, pref_timed_gd))(input.into())
}

impl crate::parsers::Parser for DurativeActionGoalDefinition {
    type Item = DurativeActionGoalDefinition;

    /// See [`parse_da_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_da_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::UnwrapValue;
    use crate::{
        AtomicFormula, GoalDefinition, Interval, Parser, PrefTimedGD, Term, TimeSpecifier, TimedGD,
        Typed, TypedList, Variable,
    };

    #[test]
    fn complex_works() {
        let input = r#"(and
                (at start (rover ?rover))
                (at start (waypoint ?from-waypoint))
                (at start (waypoint ?to-waypoint))
                (over all (can-move ?from-waypoint ?to-waypoint))
                (at start (at ?rover ?from-waypoint))
                (at start (> (battery-amount ?rover) 8)))"#;
        let (_, _gd) = DurativeActionGoalDefinition::parse(Span::new(input)).unwrap();
    }

    #[test]
    fn test_at_start() {
        assert!(
            DurativeActionGoalDefinition::parse("(at start (= x y))").is_value(
                DurativeActionGoalDefinition::Timed(PrefTimedGD::Required(TimedGD::new_at(
                    TimeSpecifier::Start,
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                )))
            )
        );
    }

    #[test]
    fn test_and_empty() {
        assert!(DurativeActionGoalDefinition::parse("(and )")
            .is_value(DurativeActionGoalDefinition::new_and([])));
    }

    #[test]
    fn test_and() {
        assert!(
            DurativeActionGoalDefinition::parse("(and (at start (= x y)) (over all (= a b)))")
                .is_value(DurativeActionGoalDefinition::new_and([
                    DurativeActionGoalDefinition::Timed(PrefTimedGD::Required(TimedGD::new_at(
                        TimeSpecifier::Start,
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    ))),
                    DurativeActionGoalDefinition::Timed(PrefTimedGD::Required(TimedGD::new_over(
                        Interval::All,
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    )))
                ]))
        );
    }

    #[test]
    fn test_forall() {
        assert!(
            DurativeActionGoalDefinition::parse("(forall (?a ?b) (at start (= a b)))").is_value(
                DurativeActionGoalDefinition::new_forall(
                    TypedList::from_iter([
                        Typed::new_object(Variable::from_str("a")),
                        Typed::new_object(Variable::from_str("b")),
                    ]),
                    DurativeActionGoalDefinition::Timed(PrefTimedGD::Required(TimedGD::new_at(
                        TimeSpecifier::Start,
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    )))
                )
            )
        );
    }
}
