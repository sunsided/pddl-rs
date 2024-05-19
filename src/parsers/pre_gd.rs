//! Provides parsers for goal definitions.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

use crate::parsers::{parens, prefix_expr, space_separated_list0, typed_list, ParseResult, Span};
use crate::parsers::{parse_pref_gd, parse_variable};
use crate::types::PreconditionGoalDefinition;
use crate::PreconditionGoalDefinitions;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_pre_gd, preamble::*};
/// # use pddl::{AtomicFormula, EqualityAtomicFormula, GoalDefinition, Literal, Preference, PreferenceName, PreferenceGD, PreconditionGoalDefinitions, PreconditionGoalDefinition, Term, Variable, Type, Typed, TypedList};
/// assert!(parse_pre_gd(Span::new("(= x y)")).is_value(
///     PreconditionGoalDefinitions::new_preference(
///         PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )
///     )
/// ));
///
/// assert!(parse_pre_gd(Span::new("(and (= x y) (= a b))")).is_value(
///     PreconditionGoalDefinitions::from_iter([
///         PreconditionGoalDefinition::Preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("x".into()),
///                     Term::Name("y".into())
///                 )
///             )
///         )),
///         PreconditionGoalDefinition::Preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("a".into()),
///                     Term::Name("b".into())
///                 )
///             )
///         ))
///     ])
/// ));
///
/// assert!(parse_pre_gd(Span::new("(forall (?a ?b) (= a b))")).is_value(
///     PreconditionGoalDefinition::new_forall(
///         TypedList::from_iter([
///             Typed::new(Variable::from_str("a"), Type::OBJECT),
///             Typed::new(Variable::from_str("b"), Type::OBJECT),
///         ]),
///         PreconditionGoalDefinition::new_preference(PreferenceGD::Goal(
///             GoalDefinition::AtomicFormula(
///                 AtomicFormula::new_equality(
///                     Term::Name("a".into()),
///                     Term::Name("b".into())
///                 )
///             )
///         )).into()
///     ).into()
/// ));
/// ```
pub fn parse_pre_gd<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, PreconditionGoalDefinitions> {
    let pref_gd = map(parse_pref_gd, |x| {
        PreconditionGoalDefinitions::from(PreconditionGoalDefinition::new_preference(x))
    });
    let and = map(
        prefix_expr("and", space_separated_list0(parse_pre_gd)),
        |x| PreconditionGoalDefinitions::from_iter(x.into_iter().flatten()),
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_pre_gd),
            )),
        ),
        |(vars, gd)| {
            PreconditionGoalDefinitions::from(PreconditionGoalDefinition::new_forall(vars, gd))
        },
    );

    alt((forall, and, pref_gd))(input.into())
}

impl crate::parsers::Parser for PreconditionGoalDefinitions {
    type Item = PreconditionGoalDefinitions;

    /// See [`parse_pre_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pre_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{
        AtomicFormula, GoalDefinition, PreconditionGoalDefinition, PreconditionGoalDefinitions,
        PreferenceGD, Term, Type, Typed, TypedList, Variable,
    };

    #[test]
    fn test_parse() {
        assert!(
            PreconditionGoalDefinitions::parse(Span::new("(= x y)")).is_value(
                PreconditionGoalDefinitions::new_preference(PreferenceGD::Goal(
                    GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))
                ))
            )
        );

        assert!(
            PreconditionGoalDefinitions::parse(Span::new("(and (= x y) (= a b))")).is_value(
                PreconditionGoalDefinitions::from_iter([
                    PreconditionGoalDefinition::Preference(PreferenceGD::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("x".into()),
                            Term::Name("y".into())
                        ))
                    )),
                    PreconditionGoalDefinition::Preference(PreferenceGD::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                ])
            )
        );

        assert!(
            PreconditionGoalDefinitions::parse(Span::new("(forall (?a ?b) (= a b))")).is_value(
                PreconditionGoalDefinition::new_forall(
                    TypedList::from_iter([
                        Typed::new(Variable::from_str("a"), Type::OBJECT),
                        Typed::new(Variable::from_str("b"), Type::OBJECT),
                    ]),
                    PreconditionGoalDefinition::new_preference(PreferenceGD::Goal(
                        GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
                            Term::Name("a".into()),
                            Term::Name("b".into())
                        ))
                    ))
                    .into()
                )
                .into()
            )
        );
    }
}
