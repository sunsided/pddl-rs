//! Provides parsers for numbers, decimals and digits.

use nom::character::complete::{char, digit1};
use nom::combinator::recognize;
use nom::multi::many_m_n;
use nom::number::complete::double;
use nom::Parser;

use crate::parsers::{failure_at, ParseResult, Span};
use crate::types::Number;

/// Parses a number, i.e. `<digit>⁺[<decimal>]`.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_number, preamble::*};
/// assert!(parse_number("0").is_value(0.0.into()));
/// assert!(parse_number("1000a").is_value(1000.0.into()));
/// assert!(parse_number("012").is_value(12.0.into()));
/// assert!(parse_number("1.234").is_value(1.234.into()));
///
/// assert!(parse_number(".0").is_err());
/// assert!(parse_number(".").is_err());
/// assert!(parse_number("-1").is_err());
///```
pub fn parse_number<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, Number> {
    let input = input.into();
    let pattern = recognize((digit1, many_m_n(0, 1, parse_decimal)));
    let (rest, value) = pattern.and_then(double).parse(input)?;
    // Too many digits overflow to infinity, which `Number` rejects. A `Failure`, not an
    // `Error`: no other token starts with a digit, and backtracking would lose the reason.
    match Number::try_new(value) {
        Ok(number) => Ok((rest, number)),
        Err(_) => Err(failure_at(input, "number out of range")),
    }
}

/// Parses a decimal, i.e. `.<digit>⁺`.
pub fn parse_decimal(input: Span) -> ParseResult<Span> {
    recognize((char('.'), digit1)).parse(input)
}

impl crate::parsers::Parser for Number {
    type Item = Number;

    /// See [`parse_number`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_number(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::number::parse_decimal;
    use crate::parsers::{Match, Span};
    use crate::{Number, Parser};

    #[test]
    fn parse_decimal_works() {
        assert!(parse_decimal(Span::new(".0")).is_exactly(".0"));
        assert!(parse_decimal(Span::new(".012")).is_exactly(".012"));
        assert!(parse_decimal(Span::new(".")).is_err());
        assert!(parse_decimal(Span::new("012")).is_err());
    }

    #[test]
    fn test_parse() {
        let (_, value) = Number::parse("1.20").unwrap();
        assert_eq!(value, Number::from(1.2));
    }

    #[test]
    fn too_many_digits_is_an_error_not_a_panic() {
        // About 1e308 still fits an f64; one more digit overflows.
        assert!(Number::from_str(&"9".repeat(308)).is_ok());
        let error = Number::from_str(&"9".repeat(309)).unwrap_err();
        assert_eq!(error.reason(), Some("number out of range"));
        assert_eq!(
            (error.offset(), error.found().map(|f| &f[..3])),
            (0, Some("999"))
        );
    }
}
