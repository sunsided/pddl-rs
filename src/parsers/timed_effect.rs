//! Provides parsers for timed effects (`<timed-effect>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{
    alt, parens, parse_assign_op_t, parse_effect_condition, parse_f_assign_da, parse_f_exp_t,
    parse_f_head, parse_time_specifier, prefix,
};
use crate::types::TimedEffect;

/// Parses a timed effect (`<timed-effect>`), i.e. `(at <time-specifier> <f-assign-da>)
/// | (at <time-specifier> <cond-effect>) | (<assign-op-t> <f-head> <f-exp-t>)`.
pub(crate) fn parse_timed_effect(node: &Node) -> R<TimedEffect> {
    alt(
        node,
        &[
            &|n| {
                prefix(n, "at", |args| {
                    Ok((
                        parse_time_specifier(args.next()?)?,
                        parse_f_assign_da(args.next()?)?,
                    ))
                })
                .map(TimedEffect::from)
            },
            &|n| {
                prefix(n, "at", |args| {
                    Ok((
                        parse_time_specifier(args.next()?)?,
                        parse_effect_condition(args.next()?)?,
                    ))
                })
                .map(TimedEffect::from)
            },
            &|n| {
                parens(n, |args| {
                    Ok((
                        parse_assign_op_t(args.next()?)?,
                        parse_f_head(args.next()?)?,
                        parse_f_exp_t(args.next()?)?,
                    ))
                })
                .map(TimedEffect::from)
            },
        ],
    )
}

crate::parsers::impl_parser!(TimedEffect => parse_timed_effect);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn it_works() {
        let input = "(at end (increase (distance-travelled) 5))";
        let _effect = TimedEffect::from_str(input).unwrap();
    }

    #[test]
    fn test_parse() {
        assert!(
            TimedEffect::from_str("(at start (= x y))").is_value(TimedEffect::conditional(
                TimeSpecifier::Start,
                EffectCondition::new(PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                )))
            ))
        );

        assert!(
            TimedEffect::from_str("(at end (assign fun-sym ?duration))").is_value(
                TimedEffect::fluent(
                    TimeSpecifier::End,
                    DurativeActionFunctionAssignment::new(
                        AssignOp::Assign,
                        FunctionHead::Simple("fun-sym".into()),
                        DurativeActionFluentExpression::Duration
                    )
                )
            )
        );

        assert!(
            TimedEffect::from_str("(increase fun-sym #t)").is_value(TimedEffect::continuous(
                TimedAssignOperator::Increase,
                FunctionHead::Simple("fun-sym".into()),
                TimedFluentExpression::Now
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(
            TimedEffect::from_str("(at start (= x y))").is_value(TimedEffect::conditional(
                TimeSpecifier::Start,
                EffectCondition::new(PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
                    EqualityAtomicFormula::new(Term::Name("x".into()), Term::Name("y".into()))
                )))
            ))
        );

        assert!(
            TimedEffect::from_str("(at end (assign fun-sym ?duration))").is_value(
                TimedEffect::fluent(
                    TimeSpecifier::End,
                    DurativeActionFunctionAssignment::new(
                        AssignOp::Assign,
                        FunctionHead::Simple("fun-sym".into()),
                        DurativeActionFluentExpression::Duration
                    )
                )
            )
        );

        assert!(
            TimedEffect::from_str("(increase fun-sym #t)").is_value(TimedEffect::continuous(
                TimedAssignOperator::Increase,
                FunctionHead::Simple("fun-sym".into()),
                TimedFluentExpression::Now
            ))
        );
    }
}
