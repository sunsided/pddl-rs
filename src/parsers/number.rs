//! Provides parsers for numbers, decimals and digits.

use crate::parsers::{ParseResult, Span};
use crate::types::Number;
use nom::character::complete::{char, digit1};
use nom::combinator::{map, recognize};
use nom::multi::many_m_n;
use nom::number::complete::float;
use nom::sequence::tuple;
use nom::Parser;

/// Parses a number, i.e. `<digit>⁺[<decimal>]`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_number, preamble::*};
/// assert!(parse_number("0".into()).is_value(0.0.into()));
/// assert!(parse_number("1000a".into()).is_value(1000.0.into()));
/// assert!(parse_number("012".into()).is_value(12.0.into()));
/// assert!(parse_number("1.234".into()).is_value(1.234.into()));
///
/// assert!(parse_number(".0".into()).is_err());
/// assert!(parse_number(".".into()).is_err());
/// assert!(parse_number("-1".into()).is_err());
///```
pub fn parse_number(input: Span) -> ParseResult<Number> {
    let pattern = recognize(tuple((digit1, many_m_n(0, 1, parse_decimal))));
    let float = pattern.and_then(float);
    map(float, Number::new)(input)
}

/// Parses a decimal, i.e. `.<digit>⁺`.
pub fn parse_decimal(input: Span) -> ParseResult<Span> {
    recognize(tuple((char('.'), digit1)))(input)
}

impl<'a> crate::parsers::Parser<'a> for Number {
    type Item = Number;

    /// See [`parse_number`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_number(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::Match;

    #[test]
    fn parse_decimal_works() {
        assert!(parse_decimal(Span::new(".0")).is_exactly(".0"));
        assert!(parse_decimal(Span::new(".012")).is_exactly(".012"));
        assert!(parse_decimal(Span::new(".")).is_err());
        assert!(parse_decimal(Span::new("012")).is_err());
    }
}
