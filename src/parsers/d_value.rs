//! Provides parsers for d-values.

use nom::branch::alt;
use nom::combinator::map;

use crate::parsers::parse_number;
use crate::parsers::{parse_f_exp, ParseResult, Span};
use crate::types::DurationValue;

/// Parses a d-value.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_d_value, preamble::*};
/// # use pddl::{BinaryOp, DurationValue, FExp, FHead, FunctionSymbol, MultiOp};
/// assert!(parse_d_value("1.23").is_value(
///     DurationValue::new_number(1.23)
/// ));
///
/// assert!(parse_d_value("fun-sym").is_value(
///     DurationValue::new_f_exp(
///         FExp::new_function(FHead::Simple("fun-sym".into()))
///     )
/// ));
///```
pub fn parse_d_value<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, DurationValue> {
    let number = map(parse_number, DurationValue::new_number);

    // :numeric-fluents
    let f_exp = map(parse_f_exp, DurationValue::new_f_exp);

    alt((number, f_exp))(input.into())
}

impl crate::parsers::Parser for DurationValue {
    type Item = DurationValue;

    /// See [`parse_d_value`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_d_value(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{DurationValue, FExp, FHead, Parser};

    #[test]
    fn test_parse() {
        assert!(DurationValue::parse("1.23").is_value(DurationValue::new_number(1.23)));

        assert!(
            DurationValue::parse("fun-sym").is_value(DurationValue::new_f_exp(FExp::new_function(
                FHead::Simple("fun-sym".into())
            )))
        );
    }
}
