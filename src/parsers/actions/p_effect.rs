//! Provides parsers for primitive effects (`<p-effect>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    alt, atomic_formula, parens, parse_assign_op, parse_f_exp, parse_f_head, parse_function_term,
    parse_term, prefix,
};
use crate::types::PrimitiveEffect;

/// Parses a primitive effect (`<p-effect>`), i.e. `<atomic formula(term)> | (not <atomic formula(term)>)
/// | (<assign-op> <f-head> <f-exp>) | (assign <function-term> <term>)
/// | (assign <function-term> undefined)`.
pub(crate) fn parse_p_effect(node: &Node) -> R<PrimitiveEffect> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "not", |args| atomic_formula(args.next()?, parse_term))
                    .map(PrimitiveEffect::not)
            },
            &|n| {
                prefix(n, "assign", |args| {
                    let term = parse_function_term(args.next()?)?;
                    args.expect("undefined")?;
                    Ok(term)
                })
                .map(|term| PrimitiveEffect::object_fluent(term, None))
            },
            &|n| {
                prefix(n, "assign", |args| {
                    Ok((
                        parse_function_term(args.next()?)?,
                        parse_term(args.next()?)?,
                    ))
                })
                .map(|(f_term, term)| PrimitiveEffect::object_fluent(f_term, Some(term)))
            },
            &|n| {
                parens(n, |args| {
                    Ok((
                        parse_assign_op(args.next()?)?,
                        parse_f_head(args.next()?)?,
                        parse_f_exp(args.next()?)?,
                    ))
                })
                .map(|(op, head, exp)| PrimitiveEffect::numeric_fluent(op, head, exp))
            },
            &|n| atomic_formula(n, parse_term).map(PrimitiveEffect::new),
        ],
    )
}

crate::parsers::impl_parser!(PrimitiveEffect => parse_p_effect);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use assert2::check;

    use crate::*;

    #[test]
    fn it_works() {
        let input = "(can-move ?from-waypoint ?to-waypoint)";
        let _effect = PrimitiveEffect::from_str(input).unwrap();
    }

    #[test]
    fn not_works() {
        let input = "(not (at B ?m))";
        check!(let Ok(PrimitiveEffect::NotAtomicFormula(_)) = PrimitiveEffect::from_str(input));
    }

    #[test]
    fn test_parse() {
        check!(
            PrimitiveEffect::from_str("(= x y)").is_value(PrimitiveEffect::AtomicFormula(
                AtomicFormula::Equality(EqualityAtomicFormula::new(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        check!(PrimitiveEffect::from_str("(not (= ?a B))").is_value(
            PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                Term::Variable("a".into()),
                Term::Name("B".into())
            )))
        ));

        check!(PrimitiveEffect::from_str("(assign fun-sym 1.23)").is_value(
            PrimitiveEffect::numeric_fluent(
                AssignOp::Assign,
                FunctionHead::new(FunctionSymbol::string("fun-sym")),
                FluentExpression::number(1.23)
            )
        ));

        check!(PrimitiveEffect::from_str("(assign fun-sym 1.23)").is_value(
            PrimitiveEffect::numeric_fluent(
                AssignOp::Assign,
                FunctionHead::new(FunctionSymbol::string("fun-sym")),
                FluentExpression::number(1.23)
            )
        ));

        check!(
            PrimitiveEffect::from_str("(assign (fun-sym) undefined)").is_value(
                PrimitiveEffect::object_fluent(
                    FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
                    None
                )
            )
        );

        check!(
            PrimitiveEffect::from_str("(assign (fun-sym) something)").is_value(
                PrimitiveEffect::object_fluent(
                    FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
                    Some(Term::Name("something".into()))
                )
            )
        );
    }

    #[test]
    fn doc_example_0() {
        check!(
            PrimitiveEffect::from_str("(= x y)").is_value(PrimitiveEffect::AtomicFormula(
                AtomicFormula::Equality(EqualityAtomicFormula::new(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        check!(PrimitiveEffect::from_str("(not (= ?a B))").is_value(
            PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                Term::Variable("a".into()),
                Term::Name("B".into())
            )))
        ));

        check!(PrimitiveEffect::from_str("(assign fun-sym 1.23)").is_value(
            PrimitiveEffect::numeric_fluent(
                AssignOp::Assign,
                FunctionHead::new(FunctionSymbol::string("fun-sym")),
                FluentExpression::number(1.23)
            )
        ));

        check!(PrimitiveEffect::from_str("(assign fun-sym 1.23)").is_value(
            PrimitiveEffect::numeric_fluent(
                AssignOp::Assign,
                FunctionHead::new(FunctionSymbol::string("fun-sym")),
                FluentExpression::number(1.23)
            )
        ));

        check!(
            PrimitiveEffect::from_str("(assign (fun-sym) undefined)").is_value(
                PrimitiveEffect::object_fluent(
                    FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
                    None
                )
            )
        );

        check!(
            PrimitiveEffect::from_str("(assign (fun-sym) something)").is_value(
                PrimitiveEffect::object_fluent(
                    FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
                    Some(Term::Name("something".into()))
                )
            )
        );
    }
}
