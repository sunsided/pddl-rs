//! Provides parsers for goal descriptions (`<GD>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    alt, atomic_formula, literal, parse_f_comp, parse_term, prefix, typed_variables,
};
use crate::types::GoalDescription;

/// Parses a goal description (`<GD>`).
///
/// ## Requirements
/// - `(not <GD>)` requires `:negative-preconditions` on atomic formulas, or
///   `:disjunctive-preconditions` otherwise.
/// - `or` and `imply` require `:disjunctive-preconditions`.
/// - `exists` requires `:existential-preconditions`, `forall` `:universal-preconditions`.
/// - Comparisons require `:numeric-fluents`.
pub(crate) fn parse_gd(node: &Node) -> R<GoalDescription> {
    alt(
        node,
        &[
            &|n| prefix(n, "and", |args| args.list0(parse_gd)).map(GoalDescription::new_and),
            &|n| prefix(n, "or", |args| args.list0(parse_gd)).map(GoalDescription::new_or),
            &|n| prefix(n, "not", |args| parse_gd(args.next()?)).map(GoalDescription::new_not),
            &|n| {
                prefix(n, "imply", |args| {
                    Ok((parse_gd(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(GoalDescription::new_imply_tuple)
            },
            &|n| {
                prefix(n, "exists", |args| {
                    Ok((typed_variables(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(GoalDescription::new_exists_tuple)
            },
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((typed_variables(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(GoalDescription::new_forall_tuple)
            },
            &|n| atomic_formula(n, parse_term).map(GoalDescription::new_atomic_formula),
            &|n| literal(n, parse_term).map(GoalDescription::new_literal),
            &|n| parse_f_comp(n).map(GoalDescription::new_f_comp),
        ],
    )
}

crate::parsers::impl_parser!(GoalDescription => parse_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            GoalDescription::from_str("(= x y)").is_value(GoalDescription::AtomicFormula(
                AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        );

        // Literal
        assert!(
            GoalDescription::from_str("(not (= x y))").is_value(GoalDescription::not(
                GoalDescription::atomic_formula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        // Conjunction (and)
        assert!(
            GoalDescription::from_str("(and (not (= x y)) (= x z))").is_value(
                GoalDescription::and([
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDescription::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                ])
            )
        );

        // Disjunction (or)
        assert!(
            GoalDescription::from_str("(or (not (= x y)) (= x z))").is_value(GoalDescription::or(
                [
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDescription::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                ]
            ))
        );

        // Implication
        assert!(
            GoalDescription::from_str("(imply (not (= x y)) (= x z))").is_value(
                GoalDescription::imply(
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDescription::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                )
            )
        );

        // Existential preconditions
        assert!(
            GoalDescription::from_str("(exists (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDescription::exists(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        // Universal preconditions
        assert!(
            GoalDescription::from_str("(forall (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDescription::forall(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        assert!(
            GoalDescription::from_str("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
                GoalDescription::fluent_comparison(FluentComparison::new(
                    BinaryComparison::Equal,
                    FluentExpression::binary_op(
                        BinaryOp::Addition,
                        FluentExpression::number(1.23),
                        FluentExpression::number(2.34),
                    ),
                    FluentExpression::binary_op(
                        BinaryOp::Addition,
                        FluentExpression::number(1.23),
                        FluentExpression::number(2.34),
                    )
                ))
            )
        );
    }

    #[test]
    fn doc_example_0() {
        // Atomic formula
        assert!(
            GoalDescription::from_str("(= x y)").is_value(GoalDescription::AtomicFormula(
                AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        );

        // Literal
        assert!(
            GoalDescription::from_str("(not (= x y))").is_value(GoalDescription::not(
                GoalDescription::atomic_formula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        // Conjunction (and)
        assert!(
            GoalDescription::from_str("(and (not (= x y)) (= x z))").is_value(
                GoalDescription::and([
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDescription::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                ])
            )
        );

        // Disjunction (or)
        assert!(
            GoalDescription::from_str("(or (not (= x y)) (= x z))").is_value(GoalDescription::or(
                [
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDescription::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                ]
            ))
        );

        // Implication
        assert!(
            GoalDescription::from_str("(imply (not (= x y)) (= x z))").is_value(
                GoalDescription::imply(
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDescription::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                )
            )
        );

        // Existential preconditions
        assert!(
            GoalDescription::from_str("(exists (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDescription::exists(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        // Universal preconditions
        assert!(
            GoalDescription::from_str("(forall (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDescription::forall(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDescription::not(GoalDescription::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        assert!(
            GoalDescription::from_str("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
                GoalDescription::fluent_comparison(FluentComparison::new(
                    BinaryComparison::Equal,
                    FluentExpression::binary_op(
                        BinaryOp::Addition,
                        FluentExpression::number(1.23),
                        FluentExpression::number(2.34),
                    ),
                    FluentExpression::binary_op(
                        BinaryOp::Addition,
                        FluentExpression::number(1.23),
                        FluentExpression::number(2.34),
                    )
                ))
            )
        );
    }
}
