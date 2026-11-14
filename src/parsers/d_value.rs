//! Provides parsers for duration values (`<d-value>`).

use crate::parsers::error::R;
use crate::parsers::sexpr::Node;
use crate::parsers::{alt, parse_f_exp, parse_number};
use crate::types::DurationValue;

/// Parses a duration value (`<d-value>`), i.e. `<number> | <f-exp>`.
pub(crate) fn parse_d_value(node: &Node) -> R<DurationValue> {
    alt(
        node,
        &[&|n| parse_number(n).map(DurationValue::new_number), &|n| {
            parse_f_exp(n).map(DurationValue::new_f_exp)
        }],
    )
}

crate::parsers::impl_parser!(DurationValue => parse_d_value);

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;

    use crate::*;

    #[test]
    fn test_parse() {
        assert!(DurationValue::from_str("1.23").is_value(DurationValue::number(1.23)));

        assert!(
            DurationValue::from_str("fun-sym").is_value(DurationValue::fluent_expression(
                FluentExpression::function(FunctionHead::Simple("fun-sym".into()))
            ))
        );
    }

    #[test]
    fn doc_example_0() {
        assert!(DurationValue::from_str("1.23").is_value(DurationValue::number(1.23)));

        assert!(
            DurationValue::from_str("fun-sym").is_value(DurationValue::fluent_expression(
                FluentExpression::function(FunctionHead::Simple("fun-sym".into()))
            ))
        );
    }
}
