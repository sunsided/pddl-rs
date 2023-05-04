//! Provides parsers for preferred goal definitions.

use crate::parsers::{
    parens, parse_con_gd, parse_pref_name, parse_variable, prefix_expr, space_separated_list0,
    typed_list, ParseResult, Span,
};
use crate::PrefConGDs;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parses preferred goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_pref_con_gd, preamble::*};
/// # use pddl::{AtomicFormula, Con2GD, ConGD, GoalDefinition, Number, PrefConGD, PrefConGDs, Term, ToTyped, Type, TypedList, Variable};
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
/// assert!(parse_pref_con_gd("(and)".into()).is_value(
///     PrefConGDs::default()
/// ));
///
/// assert!(parse_pref_con_gd("(and (at end (= x y)) (at end (not (= x z))))".into()).is_value(
///     PrefConGDs::from_iter([
///         PrefConGD::new_goal(ConGD::new_at_end(gd_a.clone())),
///         PrefConGD::new_goal(ConGD::new_at_end(gd_b.clone())),
///     ])
/// ));
///
/// assert!(parse_pref_con_gd("(forall (?x ?z) (sometime (= ?x ?z)))".into()).is_value(
///     PrefConGDs::new_forall(
///         TypedList::from_iter([
///             Variable::from("x").to_typed(Type::OBJECT),
///             Variable::from("z").to_typed(Type::OBJECT),
///         ]),
///         PrefConGDs::new_goal(ConGD::new_sometime(
///                 // gd ...
///                 # Con2GD::Goal(
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
/// assert!(parse_pref_con_gd("(at end (= x y))".into()).is_value(
///     PrefConGDs::new_goal(ConGD::AtEnd(gd_a.clone()))
/// ));
///
/// assert!(parse_pref_con_gd("(preference (at end (= x y)))".into()).is_value(
///     PrefConGDs::new_preference(None, ConGD::AtEnd(gd_a.clone()))
/// ));
///
/// assert!(parse_pref_con_gd("(preference name (at end (= x y)))".into()).is_value(
///     PrefConGDs::new_preference(Some("name".into()), ConGD::AtEnd(gd_a.clone()))
/// ));
/// ```
pub fn parse_pref_con_gd(input: Span) -> ParseResult<PrefConGDs> {
    let and = map(
        prefix_expr("and", space_separated_list0(parse_pref_con_gd)),
        |x| PrefConGDs::from_iter(x.into_iter().flatten()),
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_pref_con_gd),
            )),
        ),
        |(vars, gd)| PrefConGDs::new_forall(vars, gd),
    );

    // :preferences
    let named_preference = map(
        prefix_expr(
            "preference",
            tuple((parse_pref_name, preceded(multispace1, parse_con_gd))),
        ),
        |(name, gd)| PrefConGDs::new_preference(Some(name), gd),
    );

    // :preferences
    let unnamed_preference = map(prefix_expr("preference", parse_con_gd), |gd| {
        PrefConGDs::new_preference(None, gd)
    });

    let goal = map(parse_con_gd, PrefConGDs::new_goal);

    alt((and, forall, named_preference, unnamed_preference, goal))(input)
}

impl<'a> crate::parsers::Parser<'a> for PrefConGDs<'a> {
    type Item = PrefConGDs<'a>;

    /// See [`parse_pref_con_gd`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_pref_con_gd(input.into())
    }
}
