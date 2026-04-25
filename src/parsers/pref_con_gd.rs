//! Provides parsers for preferred goal definitions.

use crate::parsers::{
    parens, parse_con_gd, parse_pref_name, parse_variable, prefix_expr, space_separated_list0,
    typed_list, ParseResult, Span,
};
use crate::PreferenceConstraintGoalDefinitions;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

/// Parses preferred goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_pref_con_gd, preamble::*};
/// # use pddl::{AtomicFormula, ConstraintGoalDefinitionInner, ConstraintGoalDefinition, GoalDefinition, Number, PreferenceConstraintGoalDefinition, PreferenceConstraintGoalDefinitions, Term, ToTyped, Type, TypedList, Variable};
/// // (= x y)
/// let gd_a =
///     GoalDefinition::new_atomic_formula(
///         AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("y".into())
///         )
///     );
///
/// // (not (= x z))
/// let gd_b =
///     GoalDefinition::new_not(
///         GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("z".into())
///             )
///         )
///     );
///
/// assert!(parse_pref_con_gd("(and)").is_value(
///     PreferenceConstraintGoalDefinitions::default()
/// ));
///
/// assert!(parse_pref_con_gd("(and (at end (= x y)) (at end (not (= x z))))").is_value(
///     PreferenceConstraintGoalDefinitions::from_iter([
///         PreferenceConstraintGoalDefinition::new_goal(ConstraintGoalDefinition::new_at_end(gd_a.clone())),
///         PreferenceConstraintGoalDefinition::new_goal(ConstraintGoalDefinition::new_at_end(gd_b.clone())),
///     ])
/// ));
///
/// assert!(parse_pref_con_gd("(forall (?x ?z) (sometime (= ?x ?z)))").is_value(
///     PreferenceConstraintGoalDefinitions::new_forall(
///         TypedList::from_iter([
///             Variable::from("x").to_typed(Type::OBJECT),
///             Variable::from("z").to_typed(Type::OBJECT),
///         ]),
///         PreferenceConstraintGoalDefinitions::new_goal(ConstraintGoalDefinition::new_sometime(
///                 // gd ...
///                 # ConstraintGoalDefinitionInner::Goal(
///                     # GoalDefinition::new_atomic_formula(
///                     #    AtomicFormula::new_equality(
///                     #        Term::Variable("x".into()),
///                     #        Term::Variable("z".into())
///                     #    )
///                     # )
///                 # )
///             )
///         )
///     )
/// ));
///
/// assert!(parse_pref_con_gd("(at end (= x y))").is_value(
///     PreferenceConstraintGoalDefinitions::new_goal(ConstraintGoalDefinition::AtEnd(gd_a.clone()))
/// ));
///
/// assert!(parse_pref_con_gd("(preference (at end (= x y)))").is_value(
///     PreferenceConstraintGoalDefinitions::new_preference(None, ConstraintGoalDefinition::AtEnd(gd_a.clone()))
/// ));
///
/// assert!(parse_pref_con_gd("(preference name (at end (= x y)))").is_value(
///     PreferenceConstraintGoalDefinitions::new_preference(Some("name".into()), ConstraintGoalDefinition::AtEnd(gd_a.clone()))
/// ));
/// ```
pub fn parse_pref_con_gd<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, PreferenceConstraintGoalDefinitions> {
    let and = map(
        prefix_expr("and", space_separated_list0(parse_pref_con_gd)),
        |x| PreferenceConstraintGoalDefinitions::from_iter(x.into_iter().flatten()),
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            (
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_pref_con_gd),
            ),
        ),
        |(vars, gd)| PreferenceConstraintGoalDefinitions::new_forall(vars, gd),
    );

    // :preferences
    let named_preference = map(
        prefix_expr(
            "preference",
            (parse_pref_name, preceded(multispace1, parse_con_gd)),
        ),
        |(name, gd)| PreferenceConstraintGoalDefinitions::new_preference(Some(name), gd),
    );

    // :preferences
    let unnamed_preference = map(prefix_expr("preference", parse_con_gd), |gd| {
        PreferenceConstraintGoalDefinitions::new_preference(None, gd)
    });

    let goal = map(parse_con_gd, PreferenceConstraintGoalDefinitions::new_goal);

    alt((and, forall, named_preference, unnamed_preference, goal)).parse(input.into())
}

impl crate::parsers::Parser for PreferenceConstraintGoalDefinitions {
    type Item = PreferenceConstraintGoalDefinitions;

    /// See [`parse_pref_con_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pref_con_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{
        AtomicFormula, ConstraintGoalDefinition, ConstraintGoalDefinitionInner, GoalDefinition,
        PreferenceConstraintGoalDefinition, PreferenceConstraintGoalDefinitions, Term, ToTyped,
        Type, TypedList, Variable,
    };

    #[test]
    fn test_parse() {
        // (= x y)
        let gd_a = GoalDefinition::new_atomic_formula(AtomicFormula::new_equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));

        // (not (= x z))
        let gd_b = GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
            AtomicFormula::new_equality(Term::Name("x".into()), Term::Name("z".into())),
        ));

        assert!(PreferenceConstraintGoalDefinitions::parse("(and)")
            .is_value(PreferenceConstraintGoalDefinitions::default()));

        assert!(PreferenceConstraintGoalDefinitions::parse(
            "(and (at end (= x y)) (at end (not (= x z))))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::from_iter([
            PreferenceConstraintGoalDefinition::new_goal(ConstraintGoalDefinition::new_at_end(
                gd_a.clone()
            )),
            PreferenceConstraintGoalDefinition::new_goal(ConstraintGoalDefinition::new_at_end(
                gd_b.clone()
            )),
        ])));

        assert!(PreferenceConstraintGoalDefinitions::parse(
            "(forall (?x ?z) (sometime (= ?x ?z)))"
        )
        .is_value(PreferenceConstraintGoalDefinitions::new_forall(
            TypedList::from_iter([
                Variable::from("x").to_typed(Type::OBJECT),
                Variable::from("z").to_typed(Type::OBJECT),
            ]),
            PreferenceConstraintGoalDefinitions::new_goal(ConstraintGoalDefinition::new_sometime(
                ConstraintGoalDefinitionInner::Goal(GoalDefinition::new_atomic_formula(
                    AtomicFormula::new_equality(
                        Term::Variable("x".into()),
                        Term::Variable("z".into())
                    )
                ))
            ))
        )));

        assert!(
            PreferenceConstraintGoalDefinitions::parse("(at end (= x y))").is_value(
                PreferenceConstraintGoalDefinitions::new_goal(ConstraintGoalDefinition::AtEnd(
                    gd_a.clone()
                ))
            )
        );

        assert!(
            PreferenceConstraintGoalDefinitions::parse("(preference (at end (= x y)))").is_value(
                PreferenceConstraintGoalDefinitions::new_preference(
                    None,
                    ConstraintGoalDefinition::AtEnd(gd_a.clone())
                )
            )
        );

        assert!(
            PreferenceConstraintGoalDefinitions::parse("(preference name (at end (= x y)))")
                .is_value(PreferenceConstraintGoalDefinitions::new_preference(
                    Some("name".into()),
                    ConstraintGoalDefinition::AtEnd(gd_a.clone())
                ))
        );
    }
}
