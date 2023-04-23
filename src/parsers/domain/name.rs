//! Provides parsers for names.

use crate::parsers::utility::number::parse_digit;
use crate::types::domain::Name;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::is_alphabetic;
use nom::combinator::recognize;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{error_position, IResult};

/// Parses a name, i.e. `<letter> <any char>⁺`.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_name;
/// assert_eq!(parse_name("abcde"), Ok(("", "abcde".into())));
/// assert_eq!(parse_name("a-1_2"), Ok(("", "a-1_2".into())));
/// assert_eq!(parse_name("Z01"), Ok(("", "Z01".into())));
/// assert_eq!(parse_name("x-_-_"), Ok(("", "x-_-_".into())));
///
/// assert!(parse_name("").is_err());
/// assert!(parse_name(".").is_err());
/// assert!(parse_name("-abc").is_err());
/// assert!(parse_name("0124").is_err());
/// assert!(parse_name("-1").is_err());
///```
pub fn parse_name(input: &str) -> IResult<&str, Name> {
    let (remaining, name) = recognize(tuple((parse_alpha, many0(parse_any_char))))(input)?;
    Ok((remaining, Name::from(name)))
}

/// Parses a decimal, i.e. `.<digit>⁺`.
pub fn parse_any_char(input: &str) -> IResult<&str, &str> {
    recognize(alt((parse_alpha, parse_digit, tag("-"), tag("_"))))(input)
}

/// Parses a alphabetic character, i.e. `a..z | A..Z`.
pub fn parse_alpha(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        return Err(nom::Err::Error(error_position!(input, ErrorKind::Count)));
    }

    if !is_alphabetic(input.as_bytes()[0]) {
        return Err(nom::Err::Error(error_position!(input, ErrorKind::Alpha)));
    }

    Ok((&input[1..], &input[..1]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_name_works() {
        assert_eq!(parse_name("abcde"), Ok(("", "abcde".into())));
    }

    #[test]
    fn parse_any_char_works() {
        assert_eq!(parse_any_char("abc"), Ok(("bc", "a")));
        assert_eq!(parse_any_char("1"), Ok(("", "1")));
        assert_eq!(parse_any_char("-."), Ok((".", "-")));
        assert_eq!(parse_any_char("_"), Ok(("", "_")));
        assert!(parse_any_char(".").is_err());
        assert!(parse_any_char(".").is_err());
    }

    #[test]
    fn parse_alpha_works() {
        assert_eq!(parse_alpha("a"), Ok(("", "a")));
        assert_eq!(parse_alpha("ab"), Ok(("b", "a")));
        assert_eq!(parse_alpha("a1"), Ok(("1", "a")));
        assert_eq!(parse_alpha("ab1"), Ok(("b1", "a")));
        assert!(parse_alpha("").is_err());
        assert!(parse_alpha("1").is_err());
    }
}
