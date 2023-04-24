//! Provides parsers for numbers, decimals and digits.

use crate::types::Number;
use nom::character::complete::{char, digit1};
use nom::character::is_digit;
use nom::combinator::{map_res, recognize};
use nom::error::ErrorKind;
use nom::multi::many_m_n;
use nom::sequence::tuple;
use nom::{error_position, IResult};
use std::str::FromStr;

/// Parses a number, i.e. `<digit>⁺[<decimal>]`.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_number;
/// assert_eq!(parse_number("0"), Ok(("", 0.0.into())));
/// assert_eq!(parse_number("1000a"), Ok(("a", 1000.0.into())));
/// assert_eq!(parse_number("012"), Ok(("", 12.0.into())));
/// assert_eq!(parse_number("1.234"), Ok(("", 1.234.into())));
///
/// assert!(parse_number(".0").is_err());
/// assert!(parse_number(".").is_err());
/// assert!(parse_number("-1").is_err());
///```
pub fn parse_number(input: &str) -> IResult<&str, Number> {
    map_res(
        recognize(tuple((digit1, many_m_n(0, 1, parse_decimal)))),
        Number::from_str,
    )(input)
}

/// Parses a decimal, i.e. `.<digit>⁺`.
pub fn parse_decimal(input: &str) -> IResult<&str, &str> {
    recognize(tuple((char('.'), digit1)))(input)
}

/// Parses a decimal, i.e. `0..9`.
pub fn parse_digit(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        return Err(nom::Err::Error(error_position!(input, ErrorKind::Count)));
    }

    if !is_digit(input.as_bytes()[0]) {
        return Err(nom::Err::Error(error_position!(input, ErrorKind::Digit)));
    }

    Ok((&input[1..], &input[..1]))
}

impl<'a> crate::parsers::Parser<'a> for Number {
    type Item = Number;

    /// See [`parse_number`].
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_number(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal_works() {
        assert_eq!(parse_decimal(".0"), Ok(("", ".0")));
        assert_eq!(parse_decimal(".012"), Ok(("", ".012")));
        assert!(parse_decimal(".").is_err());
        assert!(parse_decimal("012").is_err());
    }

    #[test]
    fn parse_digit_works() {
        assert_eq!(parse_digit("0"), Ok(("", "0")));
        assert_eq!(parse_digit("12"), Ok(("2", "1")));
        assert_eq!(parse_digit("1a"), Ok(("a", "1")));
        assert_eq!(parse_digit("91a"), Ok(("1a", "9")));
        assert!(parse_digit("").is_err());
        assert!(parse_digit("a").is_err());
    }
}
