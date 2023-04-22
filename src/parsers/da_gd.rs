//! Provides parsers for durative action goal definitions.

use crate::parsers::{
    parens, parse_pref_timed_gd, parse_variable, prefix_expr, space_separated_list0, typed_list,
};
use crate::types::DurativeActionGoalDefinition;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_da_gd};
/// # use pddl::types::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Name, Preference, PreferenceName, PreferenceGD, PreGD, Term, Type, Typed, TypedList, Variable, DurativeActionGoalDefinition, PrefTimedGD, TimedGD, TimeSpecifier, Interval};
///
/// assert_eq!(parse_da_gd("(at start (= x y))"), Ok(("",
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
/// )));
///
/// assert_eq!(parse_da_gd("(and )"), Ok(("",
///     DurativeActionGoalDefinition::new_and([])
/// )));
///
/// assert_eq!(parse_da_gd("(and (at start (= x y)) (over all (= a b)))"), Ok(("",
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
/// )));
///
/// assert_eq!(parse_da_gd("(forall (?a ?b) (at start (= a b)))"), Ok(("",
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
/// )));
/// ```
pub fn parse_da_gd(input: &str) -> IResult<&str, DurativeActionGoalDefinition> {
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

    alt((forall, and, pref_timed_gd))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_works() {
        let input = r#"(and
                (at start (rover ?rover))
                (at start (waypoint ?from-waypoint))
                (at start (waypoint ?to-waypoint))
                (over all (can-move ?from-waypoint ?to-waypoint))
                (at start (at ?rover ?from-waypoint))
                (at start (> (battery-amount ?rover) 8)))"#;
        let (_, _gd) = parse_da_gd(input).unwrap();
    }
}
