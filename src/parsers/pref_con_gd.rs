//! Provides parsers for preferred goal definitions.

use crate::parsers::{
    parens, parse_con_gd, parse_pref_name, parse_variable, prefix_expr, space_separated_list0,
    typed_list,
};
use crate::types::PrefConGD;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses preferred goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_pref_con_gd;
/// # use pddl::types::{AtomicFormula, Con2GD, ConGD, GoalDefinition, Number, PrefConGD, Term, ToTyped, Type, TypedList, Variable};
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
/// assert_eq!(parse_pref_con_gd("(and)"), Ok(("",
///     PrefConGD::new_and([])
/// )));
///
/// assert_eq!(parse_pref_con_gd("(and (at end (= x y)) (at end (not (= x z))))"), Ok(("",
///     PrefConGD::new_and([
///         PrefConGD::new_goal(ConGD::new_at_end(gd_a.clone())),
///         PrefConGD::new_goal(ConGD::new_at_end(gd_b.clone())),
///     ])
/// )));
///
/// assert_eq!(parse_pref_con_gd("(forall (?x ?z) (sometime (= ?x ?z)))"), Ok(("",
///     PrefConGD::new_forall(
///         TypedList::from_iter([
///             Variable::from("x").to_typed(Type::OBJECT),
///             Variable::from("z").to_typed(Type::OBJECT),
///         ]),
///         PrefConGD::new_goal(ConGD::new_sometime(
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
/// )));
///
/// assert_eq!(parse_pref_con_gd("(at end (= x y))"), Ok(("",
///     PrefConGD::new_goal(ConGD::AtEnd(gd_a.clone()))
/// )));
///
/// assert_eq!(parse_pref_con_gd("(preference (at end (= x y)))"), Ok(("",
///     PrefConGD::new_preference(None, ConGD::AtEnd(gd_a.clone()))
/// )));
///
/// assert_eq!(parse_pref_con_gd("(preference name (at end (= x y)))"), Ok(("",
///     PrefConGD::new_preference(Some("name".into()), ConGD::AtEnd(gd_a.clone()))
/// )));
/// ```
pub fn parse_pref_con_gd(input: &str) -> IResult<&str, PrefConGD> {
    let and = map(
        prefix_expr("and", space_separated_list0(parse_pref_con_gd)),
        PrefConGD::new_and,
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
        |(vars, gd)| PrefConGD::new_forall(vars, gd),
    );

    // :preferences
    let named_preference = map(
        prefix_expr(
            "preference",
            tuple((parse_pref_name, preceded(multispace1, parse_con_gd))),
        ),
        |(name, gd)| PrefConGD::new_preference(Some(name), gd),
    );

    // :preferences
    let unnamed_preference = map(prefix_expr("preference", parse_con_gd), |gd| {
        PrefConGD::new_preference(None, gd)
    });

    let goal = map(parse_con_gd, PrefConGD::new_goal);

    alt((and, forall, named_preference, unnamed_preference, goal))(input)
}

impl<'a> crate::parsers::Parser<'a> for PrefConGD<'a> {
    type Item = PrefConGD<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_pref_con_gd(input)
    }
}
