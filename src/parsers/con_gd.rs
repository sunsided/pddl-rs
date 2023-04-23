//! Provides parsers for conditional goal definitions.

use crate::parsers::{
    parens, parse_gd, parse_number, parse_variable, prefix_expr, space_separated_list0, typed_list,
};
use crate::types::{Con2GD, ConGD};
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser combinator that parses conditional effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_con_gd;
/// # use pddl::types::{AtomicFormula, Con2GD, ConGD, GoalDefinition, Number, Term, ToTyped, Type, TypedList, Variable};
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
///
/// assert_eq!(parse_con_gd("(and (at end (= x y)) (at end (not (= x z))))"), Ok(("",
///     ConGD::new_and([
///         ConGD::new_at_end(gd_a.clone()),
///         ConGD::new_at_end(gd_b.clone()),
///     ])
/// )));
///
/// assert_eq!(parse_con_gd("(forall (?x ?z) (sometime (= ?x ?z)))"), Ok(("",
///     ConGD::new_for_all(
///         TypedList::from_iter([
///             Variable::from("x").to_typed(Type::OBJECT),
///             Variable::from("z").to_typed(Type::OBJECT),
///         ]),
///         ConGD::new_sometime(
///             Con2GD::Goal(
///                 // gd ...
///                 # GoalDefinition::new_atomic_formula(
///                 #    AtomicFormula::new_equality(
///                 #        Term::Variable("x".into()),
///                 #        Term::Variable("z".into())
///                 #    )
///                 # )
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_con_gd("(at end (= x y))"), Ok(("",
///     ConGD::AtEnd(gd_a.clone())
/// )));
///
/// assert_eq!(parse_con_gd("(always (= x y))"), Ok(("",
///     ConGD::Always(Con2GD::new_goal(gd_a.clone()))
/// )));
///
/// assert_eq!(parse_con_gd("(sometime (= x y))"), Ok(("",
///     ConGD::Sometime(Con2GD::new_goal(gd_a.clone()))
/// )));
///
/// assert_eq!(parse_con_gd("(within 10 (= x y))"), Ok(("",
///     ConGD::Within(
///         Number::from(10),
///         Con2GD::new_goal(gd_a.clone())
///     )
/// )));
///
/// assert_eq!(parse_con_gd("(at-most-once (= x y))"), Ok(("",
///     ConGD::AtMostOnce(Con2GD::new_goal(gd_a.clone()))
/// )));
///
/// assert_eq!(parse_con_gd("(sometime-after (= x y) (not (= x z)))"), Ok(("",
///     ConGD::SometimeAfter(
///         Con2GD::new_goal(gd_a.clone()),
///         Con2GD::new_goal(gd_b.clone())
///     )
/// )));
///
/// assert_eq!(parse_con_gd("(sometime-before (= x y) (not (= x z)))"), Ok(("",
///     ConGD::SometimeBefore(
///         Con2GD::new_goal(gd_a.clone()),
///         Con2GD::new_goal(gd_b.clone())
///     )
/// )));
///
/// assert_eq!(parse_con_gd("(always-within 10 (= x y) (not (= x z)))"), Ok(("",
///     ConGD::AlwaysWithin(
///         Number::from(10),
///         Con2GD::new_goal(gd_a.clone()),
///         Con2GD::new_goal(gd_b.clone())
///     )
/// )));
///
/// assert_eq!(parse_con_gd("(hold-during 10 20 (= x y))"), Ok(("",
///     ConGD::HoldDuring(
///         Number::from(10),
///         Number::from(20),
///         Con2GD::new_goal(gd_a.clone())
///     )
/// )));
///
/// assert_eq!(parse_con_gd("(hold-after 10 (= x y))"), Ok(("",
///     ConGD::HoldAfter(
///         Number::from(10),
///         Con2GD::new_goal(gd_a.clone())
///     )
/// )));
/// ```
///
/// Conditional goal definitions can be nested:
///
/// ```
/// # use pddl::parsers::parse_con_gd;
/// # use pddl::types::{AtomicFormula, Con2GD, ConGD, GoalDefinition, Number, Term};
/// # // (= x y)
/// # let gd =
/// #    GoalDefinition::new_atomic_formula(
/// #        AtomicFormula::new_equality(
/// #            Term::Name("x".into()),
/// #            Term::Name("y".into())
/// #        )
/// #    );
///
/// let input = "(within 10 (at-most-once (= x y)))";
/// assert_eq!(parse_con_gd(input), Ok(("",
///     ConGD::new_within(
///         Number::from(10),
///         Con2GD::new_nested(
///             ConGD::new_at_most_once(
///                 Con2GD::new_goal(
///                     // gd ...
///                     # gd
///                 )
///             )
///         )
///     )
/// )));
/// ```
pub fn parse_con_gd(input: &str) -> IResult<&str, ConGD> {
    let and = map(
        prefix_expr("and", space_separated_list0(parse_con_gd)),
        ConGD::new_and,
    );

    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_con_gd),
            )),
        ),
        |(vars, gd)| ConGD::new_for_all(vars, gd),
    );

    let at_end = map(prefix_expr("at end", parse_gd), ConGD::new_at_end);

    let always = map(prefix_expr("always", parse_con2_gd), ConGD::new_always);

    let sometime = map(prefix_expr("sometime", parse_con2_gd), ConGD::new_sometime);

    let within = map(
        prefix_expr(
            "within",
            tuple((parse_number, preceded(multispace1, parse_con2_gd))),
        ),
        |(num, gd)| ConGD::new_within(num, gd),
    );

    let at_most_once = map(
        prefix_expr("at-most-once", parse_con2_gd),
        ConGD::new_at_most_once,
    );

    let sometime_after = map(
        prefix_expr(
            "sometime-after",
            tuple((parse_con2_gd, preceded(multispace1, parse_con2_gd))),
        ),
        |(a, b)| ConGD::new_sometime_after(a, b),
    );

    let sometime_before = map(
        prefix_expr(
            "sometime-before",
            tuple((parse_con2_gd, preceded(multispace1, parse_con2_gd))),
        ),
        |(a, b)| ConGD::new_sometime_before(a, b),
    );

    let always_within = map(
        prefix_expr(
            "always-within",
            tuple((
                parse_number,
                preceded(multispace1, parse_con2_gd),
                preceded(multispace1, parse_con2_gd),
            )),
        ),
        |(num, a, b)| ConGD::new_always_within(num, a, b),
    );

    let hold_during = map(
        prefix_expr(
            "hold-during",
            tuple((
                parse_number,
                preceded(multispace1, parse_number),
                preceded(multispace1, parse_con2_gd),
            )),
        ),
        |(t0, t1, gd)| ConGD::new_hold_during(t0, t1, gd),
    );

    let hold_after = map(
        prefix_expr(
            "hold-after",
            tuple((parse_number, preceded(multispace1, parse_con2_gd))),
        ),
        |(time, gd)| ConGD::new_hold_after(time, gd),
    );

    alt((
        and,
        forall,
        at_end,
        always,
        sometime,
        within,
        at_most_once,
        sometime_after,
        sometime_before,
        always_within,
        hold_during,
        hold_after,
    ))(input)
}

fn parse_con2_gd(input: &str) -> IResult<&str, Con2GD> {
    let gd = map(parse_gd, Con2GD::new_goal);

    // TODO: Add crate feature to allow this to be forbidden if unsupported by the application.
    let con_gd = map(parse_con_gd, Con2GD::new_nested);
    alt((gd, con_gd))(input)
}
