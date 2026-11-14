//! Provides parsers for goal definitions.

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    alt, atomic_formula, literal, parse_f_comp, parse_term, prefix, typed_variables,
};
use crate::types::GoalDefinition;

/// Parses a goal definition.
///
/// ## Requirements
/// - `(not <GD>)` requires `:negative-preconditions` on atomic formulas, or
///   `:disjunctive-preconditions` otherwise.
/// - `or` and `imply` require `:disjunctive-preconditions`.
/// - `exists` requires `:existential-preconditions`, `forall` `:universal-preconditions`.
/// - Comparisons require `:numeric-fluents`.
pub(crate) fn parse_gd(node: &Node) -> R<GoalDefinition> {
    alt(
        node,
        &[
            &|n| prefix(n, "and", |args| args.list0(parse_gd)).map(GoalDefinition::new_and),
            &|n| prefix(n, "or", |args| args.list0(parse_gd)).map(GoalDefinition::new_or),
            &|n| prefix(n, "not", |args| parse_gd(args.next()?)).map(GoalDefinition::new_not),
            &|n| {
                prefix(n, "imply", |args| {
                    Ok((parse_gd(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(GoalDefinition::new_imply_tuple)
            },
            &|n| {
                prefix(n, "exists", |args| {
                    Ok((typed_variables(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(GoalDefinition::new_exists_tuple)
            },
            &|n| {
                prefix(n, "forall", |args| {
                    Ok((typed_variables(args.next()?)?, parse_gd(args.next()?)?))
                })
                .map(GoalDefinition::new_forall_tuple)
            },
            &|n| atomic_formula(n, parse_term).map(GoalDefinition::new_atomic_formula),
            &|n| literal(n, parse_term).map(GoalDefinition::new_literal),
            &|n| parse_f_comp(n).map(GoalDefinition::new_f_comp),
        ],
    )
}

crate::parsers::impl_parser!(GoalDefinition => parse_gd);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(
            GoalDefinition::from_str("(= x y)").is_value(GoalDefinition::AtomicFormula(
                AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        );

        // Literal
        assert!(
            GoalDefinition::from_str("(not (= x y))").is_value(GoalDefinition::not(
                GoalDefinition::atomic_formula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        // Conjunction (and)
        assert!(
            GoalDefinition::from_str("(and (not (= x y)) (= x z))").is_value(GoalDefinition::and(
                [
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                ]
            ))
        );

        // Disjunction (or)
        assert!(
            GoalDefinition::from_str("(or (not (= x y)) (= x z))").is_value(GoalDefinition::or([
                GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))),
                GoalDefinition::AtomicFormula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("z".into())
                ))
            ]))
        );

        // Implication
        assert!(
            GoalDefinition::from_str("(imply (not (= x y)) (= x z))").is_value(
                GoalDefinition::imply(
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                )
            )
        );

        // Existential preconditions
        assert!(
            GoalDefinition::from_str("(exists (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDefinition::exists(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        // Universal preconditions
        assert!(
            GoalDefinition::from_str("(forall (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDefinition::forall(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        assert!(
            GoalDefinition::from_str("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
                GoalDefinition::fluent_comparison(FluentComparison::new(
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
            GoalDefinition::from_str("(= x y)").is_value(GoalDefinition::AtomicFormula(
                AtomicFormula::equality(Term::Name("x".into()), Term::Name("y".into()))
            ))
        );

        // Literal
        assert!(
            GoalDefinition::from_str("(not (= x y))").is_value(GoalDefinition::not(
                GoalDefinition::atomic_formula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        // Conjunction (and)
        assert!(
            GoalDefinition::from_str("(and (not (= x y)) (= x z))").is_value(GoalDefinition::and(
                [
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                ]
            ))
        );

        // Disjunction (or)
        assert!(
            GoalDefinition::from_str("(or (not (= x y)) (= x z))").is_value(GoalDefinition::or([
                GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))),
                GoalDefinition::AtomicFormula(AtomicFormula::equality(
                    Term::Name("x".into()),
                    Term::Name("z".into())
                ))
            ]))
        );

        // Implication
        assert!(
            GoalDefinition::from_str("(imply (not (= x y)) (= x z))").is_value(
                GoalDefinition::imply(
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("y".into())
                    ))),
                    GoalDefinition::AtomicFormula(AtomicFormula::equality(
                        Term::Name("x".into()),
                        Term::Name("z".into())
                    ))
                )
            )
        );

        // Existential preconditions
        assert!(
            GoalDefinition::from_str("(exists (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDefinition::exists(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        // Universal preconditions
        assert!(
            GoalDefinition::from_str("(forall (?x ?y) (not (= ?x ?y)))").is_value(
                GoalDefinition::forall(
                    TypedList::from_iter([
                        Variable::new_string("x").into(),
                        Variable::new_string("y").into()
                    ]),
                    GoalDefinition::not(GoalDefinition::atomic_formula(AtomicFormula::equality(
                        Term::Variable("x".into()),
                        Term::Variable("y".into())
                    )))
                )
            )
        );

        assert!(
            GoalDefinition::from_str("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
                GoalDefinition::fluent_comparison(FluentComparison::new(
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
