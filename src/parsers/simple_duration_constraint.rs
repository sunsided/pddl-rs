//! Provides parsers for simple duration constraints (`<simple-duration-constraint>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parens, parse_d_op, parse_d_value, parse_time_specifier, prefix};
use crate::types::SimpleDurationConstraint;

/// Parses a simple duration constraint (`<simple-duration-constraint>`), i.e. `(<d-op> ?duration <d-value>)
/// | (at <time-specifier> <simple-duration-constraint>)`.
pub(crate) fn parse_simple_duration_constraint(node: &Node) -> R<SimpleDurationConstraint> {
    alt(
        node,
        &[
            &|n| {
                parens(n, |args| {
                    let op = parse_d_op(args.next()?)?;
                    args.expect("?duration")?;
                    Ok((op, parse_d_value(args.next()?)?))
                })
                .map(SimpleDurationConstraint::from)
            },
            &|n| {
                prefix(n, "at", |args| {
                    Ok((
                        parse_time_specifier(args.next()?)?,
                        parse_simple_duration_constraint(args.next()?)?,
                    ))
                })
                .map(SimpleDurationConstraint::from)
            },
        ],
    )
}

crate::parsers::impl_parser!(SimpleDurationConstraint => parse_simple_duration_constraint);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        let input = "(>= ?duration 1.23)";
        assert!(
            SimpleDurationConstraint::from_str(input).is_value(SimpleDurationConstraint::op(
                DurationOperator::GreaterOrEqual,
                DurationValue::number(1.23)
            ))
        );

        let input = "(at end (<= ?duration 1.23))";
        assert!(
            SimpleDurationConstraint::from_str(input).is_value(SimpleDurationConstraint::at(
                TimeSpecifier::End,
                SimpleDurationConstraint::Op(
                    DurationOperator::LessThanOrEqual,
                    DurationValue::number(1.23)
                )
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        let input = "(>= ?duration 1.23)";
        assert!(
            SimpleDurationConstraint::from_str(input).is_value(SimpleDurationConstraint::op(
                DurationOperator::GreaterOrEqual,
                DurationValue::number(1.23)
            ))
        );

        let input = "(at end (<= ?duration 1.23))";
        assert!(
            SimpleDurationConstraint::from_str(input).is_value(SimpleDurationConstraint::at(
                TimeSpecifier::End,
                SimpleDurationConstraint::Op(
                    DurationOperator::LessThanOrEqual,
                    DurationValue::number(1.23)
                )
            ))
        );
    }
}
