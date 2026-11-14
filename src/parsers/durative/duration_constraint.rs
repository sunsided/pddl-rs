//! Provides parsers for duration constraints (`<duration-constraint>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, empty_or, parse_one, parse_simple_duration_constraint, prefix};
use crate::types::DurationConstraint;

/// Parses a duration constraint (`<duration-constraint>`), i.e. `() | <simple-duration-constraint>
/// | (and <simple-duration-constraint>⁺)`. The empty list `()` is `None`.
pub(crate) fn parse_duration_constraint(node: &Node) -> R<Option<DurationConstraint>> {
    empty_or(node, |node| {
        alt(
            node,
            &[
                &|n| parse_simple_duration_constraint(n).map(DurationConstraint::from),
                &|n| {
                    prefix(n, "and", |args| {
                        args.list1(parse_simple_duration_constraint)
                    })
                    .map(DurationConstraint::from_iter)
                },
            ],
        )
    })
}

impl crate::parsers::Parser for DurationConstraint {
    type Item = Option<DurationConstraint>;

    fn from_str(input: &str) -> Result<Self::Item, crate::parsers::ParseError> {
        parse_one(input, parse_duration_constraint)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "()";
        assert!(DurationConstraint::from_str(input).is_value(None));

        let input = "(= ?duration 5)";
        assert!(
            DurationConstraint::from_str(input).is_value(Some(DurationConstraint::new(
                SimpleDurationConstraint::Op(
                    DurationOperator::Equal,
                    DurationValue::Number(5.into())
                )
            )))
        );

        let input = "(at end (<= ?duration 1.23))";
        assert!(
            DurationConstraint::from_str(input).is_value(Some(DurationConstraint::new(
                SimpleDurationConstraint::new_at(
                    TimeSpecifier::End,
                    SimpleDurationConstraint::Op(
                        DurationOperator::LessThanOrEqual,
                        DurationValue::Number(1.23.into())
                    )
                )
            )))
        );

        let input = "(and (at end (<= ?duration 1.23)) (>= ?duration 1.0))";
        assert!(
            DurationConstraint::from_str(input).is_value(Some(DurationConstraint::new_all([
                SimpleDurationConstraint::new_at(
                    TimeSpecifier::End,
                    SimpleDurationConstraint::Op(
                        DurationOperator::LessThanOrEqual,
                        DurationValue::Number(1.23.into())
                    )
                ),
                SimpleDurationConstraint::new_op(
                    DurationOperator::GreaterOrEqual,
                    DurationValue::Number(1.0.into())
                )
            ])))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "()";
        assert!(DurationConstraint::from_str(input).is_value(None));

        let input = "(= ?duration 5)";
        assert!(
            DurationConstraint::from_str(input).is_value(Some(DurationConstraint::new(
                SimpleDurationConstraint::Op(
                    DurationOperator::Equal,
                    DurationValue::Number(5.into())
                )
            )))
        );

        let input = "(at end (<= ?duration 1.23))";
        assert!(
            DurationConstraint::from_str(input).is_value(Some(DurationConstraint::new(
                SimpleDurationConstraint::new_at(
                    TimeSpecifier::End,
                    SimpleDurationConstraint::Op(
                        DurationOperator::LessThanOrEqual,
                        DurationValue::Number(1.23.into())
                    )
                )
            )))
        );

        let input = "(and (at end (<= ?duration 1.23)) (>= ?duration 1.0))";
        assert!(
            DurationConstraint::from_str(input).is_value(Some(DurationConstraint::new_all([
                SimpleDurationConstraint::new_at(
                    TimeSpecifier::End,
                    SimpleDurationConstraint::Op(
                        DurationOperator::LessThanOrEqual,
                        DurationValue::Number(1.23.into())
                    )
                ),
                SimpleDurationConstraint::new_op(
                    DurationOperator::GreaterOrEqual,
                    DurationValue::Number(1.0.into())
                )
            ])))
        );
    }
}
