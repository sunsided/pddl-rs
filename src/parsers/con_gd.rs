//! Provides parsers for conditional goal definitions.

use crate::parsers::{
    parens, parse_gd, parse_number, parse_variable, prefix_expr, space_separated_list0, typed_list,
    ParseResult, Span,
};
use crate::types::{ConstraintGoalDefinition, ConstraintGoalDefinitionInner};
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

/// Parses conditional goal definitions.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_con_gd, preamble::*};
/// # use pddl::{AtomicFormula, ConstraintGoalDefinitionInner, ConstraintGoalDefinition, GoalDefinition, Number, Term, ToTyped, Type, TypedList, Variable};
/// // (= x y)
/// let gd_a =
///     GoalDefinition::atomic_formula(
///         AtomicFormula::equality(
///             Term::Name("x".into()),
///             Term::Name("y".into())
///         )
///     );
///
/// // (not (= x z))
/// let gd_b =
///     GoalDefinition::not(
///         GoalDefinition::atomic_formula(
///             AtomicFormula::equality(
///                 Term::Name("x".into()),
///                 Term::Name("z".into())
///             )
///         )
///     );
///
/// assert!(parse_con_gd("(and)").is_value(
///     ConstraintGoalDefinition::and([])
/// ));
///
/// assert!(parse_con_gd("(and (at end (= x y)) (at end (not (= x z))))").is_value(
///     ConstraintGoalDefinition::and([
///         ConstraintGoalDefinition::at_end(gd_a.clone()),
///         ConstraintGoalDefinition::at_end(gd_b.clone()),
///     ])
/// ));
///
/// assert!(parse_con_gd("(forall (?x ?z) (sometime (= ?x ?z)))").is_value(
///     ConstraintGoalDefinition::forall(
///         TypedList::from_iter([
///             Variable::from("x").to_typed(Type::OBJECT),
///             Variable::from("z").to_typed(Type::OBJECT),
///         ]),
///         ConstraintGoalDefinition::sometime(
///             ConstraintGoalDefinitionInner::Goal(
///                 // gd ...
///                 # GoalDefinition::atomic_formula(
///                 #    AtomicFormula::equality(
///                 #        Term::Variable("x".into()),
///                 #        Term::Variable("z".into())
///                 #    )
///                 # )
///             )
///         )
///     )
/// ));
///
/// assert!(parse_con_gd("(at end (= x y))").is_value(
///     ConstraintGoalDefinition::AtEnd(gd_a.clone())
/// ));
///
/// assert!(parse_con_gd("(always (= x y))").is_value(
///     ConstraintGoalDefinition::Always(ConstraintGoalDefinitionInner::goal(gd_a.clone()))
/// ));
///
/// assert!(parse_con_gd("(sometime (= x y))").is_value(
///     ConstraintGoalDefinition::Sometime(ConstraintGoalDefinitionInner::goal(gd_a.clone()))
/// ));
///
/// assert!(parse_con_gd("(within 10 (= x y))").is_value(
///     ConstraintGoalDefinition::Within(
///         Number::from(10),
///         ConstraintGoalDefinitionInner::goal(gd_a.clone())
///     )
/// ));
///
/// assert!(parse_con_gd("(at-most-once (= x y))").is_value(
///     ConstraintGoalDefinition::AtMostOnce(ConstraintGoalDefinitionInner::goal(gd_a.clone()))
/// ));
///
/// assert!(parse_con_gd("(sometime-after (= x y) (not (= x z)))").is_value(
///     ConstraintGoalDefinition::SometimeAfter(
///         ConstraintGoalDefinitionInner::goal(gd_a.clone()),
///         ConstraintGoalDefinitionInner::goal(gd_b.clone())
///     )
/// ));
///
/// assert!(parse_con_gd("(sometime-before (= x y) (not (= x z)))").is_value(
///     ConstraintGoalDefinition::SometimeBefore(
///         ConstraintGoalDefinitionInner::goal(gd_a.clone()),
///         ConstraintGoalDefinitionInner::goal(gd_b.clone())
///     )
/// ));
///
/// assert!(parse_con_gd("(always-within 10 (= x y) (not (= x z)))").is_value(
///     ConstraintGoalDefinition::AlwaysWithin(
///         Number::from(10),
///         ConstraintGoalDefinitionInner::goal(gd_a.clone()),
///         ConstraintGoalDefinitionInner::goal(gd_b.clone())
///     )
/// ));
///
/// assert!(parse_con_gd("(hold-during 10 20 (= x y))").is_value(
///     ConstraintGoalDefinition::HoldDuring(
///         Number::from(10),
///         Number::from(20),
///         ConstraintGoalDefinitionInner::goal(gd_a.clone())
///     )
/// ));
///
/// assert!(parse_con_gd("(hold-after 10 (= x y))").is_value(
///     ConstraintGoalDefinition::HoldAfter(
///         Number::from(10),
///         ConstraintGoalDefinitionInner::goal(gd_a.clone())
///     )
/// ));
/// ```
///
/// Conditional goal definitions can be nested:
///
/// ```
/// # use pddl::parsers::{parse_con_gd, preamble::*};
/// # use pddl::{AtomicFormula, ConstraintGoalDefinitionInner, ConstraintGoalDefinition, GoalDefinition, Number, Term};
/// # // (= x y)
/// # let gd =
/// #    GoalDefinition::atomic_formula(
/// #        AtomicFormula::equality(
/// #            Term::Name("x".into()),
/// #            Term::Name("y".into())
/// #        )
/// #    );
///
/// let input = "(within 10 (at-most-once (= x y)))";
/// assert!(parse_con_gd(input).is_value(
///     ConstraintGoalDefinition::within(
///         Number::from(10),
///         ConstraintGoalDefinitionInner::nested(
///             ConstraintGoalDefinition::at_most_once(
///                 ConstraintGoalDefinitionInner::goal(
///                     // gd ...
///                     # gd
///                 )
///             )
///         )
///     )
/// ));
/// ```
pub fn parse_con_gd<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, ConstraintGoalDefinition> {
    let and = map(
        prefix_expr("and", space_separated_list0(parse_con_gd)),
        ConstraintGoalDefinition::new_and,
    );

    let forall = map(
        prefix_expr(
            "forall",
            (
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_con_gd),
            ),
        ),
        |(vars, gd)| ConstraintGoalDefinition::forall(vars, gd),
    );

    let at_end = map(
        prefix_expr("at end", parse_gd),
        ConstraintGoalDefinition::new_at_end,
    );

    let always = map(
        prefix_expr("always", parse_con2_gd),
        ConstraintGoalDefinition::new_always,
    );

    let sometime = map(
        prefix_expr("sometime", parse_con2_gd),
        ConstraintGoalDefinition::new_sometime,
    );

    let within = map(
        prefix_expr(
            "within",
            (parse_number, preceded(multispace1, parse_con2_gd)),
        ),
        |(num, gd)| ConstraintGoalDefinition::within(num, gd),
    );

    let at_most_once = map(
        prefix_expr("at-most-once", parse_con2_gd),
        ConstraintGoalDefinition::new_at_most_once,
    );

    let sometime_after = map(
        prefix_expr(
            "sometime-after",
            (parse_con2_gd, preceded(multispace1, parse_con2_gd)),
        ),
        |(a, b)| ConstraintGoalDefinition::sometime_after(a, b),
    );

    let sometime_before = map(
        prefix_expr(
            "sometime-before",
            (parse_con2_gd, preceded(multispace1, parse_con2_gd)),
        ),
        |(a, b)| ConstraintGoalDefinition::sometime_before(a, b),
    );

    let always_within = map(
        prefix_expr(
            "always-within",
            (
                parse_number,
                preceded(multispace1, parse_con2_gd),
                preceded(multispace1, parse_con2_gd),
            ),
        ),
        |(num, a, b)| ConstraintGoalDefinition::always_within(num, a, b),
    );

    let hold_during = map(
        prefix_expr(
            "hold-during",
            (
                parse_number,
                preceded(multispace1, parse_number),
                preceded(multispace1, parse_con2_gd),
            ),
        ),
        |(t0, t1, gd)| ConstraintGoalDefinition::hold_during(t0, t1, gd),
    );

    let hold_after = map(
        prefix_expr(
            "hold-after",
            (parse_number, preceded(multispace1, parse_con2_gd)),
        ),
        |(time, gd)| ConstraintGoalDefinition::hold_after(time, gd),
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
    ))
    .parse(input.into())
}

fn parse_con2_gd<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, ConstraintGoalDefinitionInner> {
    let gd = map(parse_gd, ConstraintGoalDefinitionInner::new_goal);

    // TODO: Add crate feature to allow this to be forbidden if unsupported by the application.
    let con_gd = map(parse_con_gd, ConstraintGoalDefinitionInner::new_nested);
    alt((gd, con_gd)).parse(input.into())
}

impl crate::parsers::Parser for ConstraintGoalDefinition {
    type Item = ConstraintGoalDefinition;

    /// See [`parse_con_gd`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_con_gd(input)
    }
}

impl crate::parsers::Parser for ConstraintGoalDefinitionInner {
    type Item = ConstraintGoalDefinitionInner;

    /// Parses a constraint goal definition.
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_con2_gd(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::preamble::*;
    use crate::{
        AtomicFormula, ConstraintGoalDefinition, ConstraintGoalDefinitionInner, GoalDefinition,
        Number, Term,
    };

    #[test]
    fn test_parse() {
        let gd = GoalDefinition::atomic_formula(AtomicFormula::equality(
            Term::Name("x".into()),
            Term::Name("y".into()),
        ));

        let input = "(within 10 (at-most-once (= x y)))";
        assert!(
            ConstraintGoalDefinition::parse(input).is_value(ConstraintGoalDefinition::within(
                Number::from(10),
                ConstraintGoalDefinitionInner::nested(ConstraintGoalDefinition::at_most_once(
                    ConstraintGoalDefinitionInner::goal(gd.clone())
                ))
            ))
        );

        let input = "(within 10 (at-most-once (= x y)))";
        assert!(ConstraintGoalDefinitionInner::parse(input).is_value(
            ConstraintGoalDefinitionInner::Nested(Box::new(ConstraintGoalDefinition::within(
                Number::from(10),
                ConstraintGoalDefinitionInner::nested(ConstraintGoalDefinition::at_most_once(
                    ConstraintGoalDefinitionInner::goal(gd)
                ))
            )))
        ));
    }

    #[test]
    fn test_parse_sometime() {
        assert!(
            ConstraintGoalDefinition::parse("(sometime (= x y))").is_value(
                ConstraintGoalDefinition::sometime(ConstraintGoalDefinitionInner::goal(
                    GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into()),
                    ))
                ))
            )
        );
    }

    #[test]
    fn test_parse_sometime_after() {
        assert!(
            ConstraintGoalDefinition::parse("(sometime-after (= x y) (= a b))").is_value(
                ConstraintGoalDefinition::sometime_after(
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
                    )),
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("a".into()), Term::Name("b".into()))
                    ))
                )
            )
        );
    }

    #[test]
    fn test_parse_hold_during() {
        assert!(
            ConstraintGoalDefinition::parse("(hold-during 5 10 (= x y))").is_value(
                ConstraintGoalDefinition::hold_during(
                    Number::from(5),
                    Number::from(10),
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
                    ))
                )
            )
        );
    }

    #[test]
    fn test_parse_hold_after() {
        assert!(
            ConstraintGoalDefinition::parse("(hold-after 5 (= x y))").is_value(
                ConstraintGoalDefinition::hold_after(
                    Number::from(5),
                    ConstraintGoalDefinitionInner::goal(GoalDefinition::atomic_formula(
                        AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
                    ))
                )
            )
        );
    }
}
