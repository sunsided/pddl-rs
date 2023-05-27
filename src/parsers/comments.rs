use crate::parsers::{ParseResult, Span};
use nom::character::complete::{char, multispace0};
use nom::combinator::opt;
use nom::sequence::{terminated, tuple};
use nom::{bytes::complete::is_not, combinator::value, sequence::pair};

/// Parses a comment and swallows trailing whitespace / newline.
///
/// ## Example
///
/// Comments are parsed and trailing whitespace is suppressed:
///
/// ```
/// # use pddl::parsers::{ignore_eol_comment};
/// let input = "; nothing\n\t; and this line\n\tthen more";
///
/// let (remainder, comment) = ignore_eol_comment(input).unwrap();
///
/// assert_eq!(comment, ());
/// assert_eq!(remainder.to_ascii_lowercase().trim(), "then more");
/// ```
///
/// If no comments are found, the text parses as expected, although leading whitespace is still suppressed.
///
/// ```
/// # use pddl::parsers::{ignore_eol_comment};
/// let input = "then more";
///
/// let (remainder, comment) = ignore_eol_comment(input).unwrap();
///
/// assert_eq!(comment, ());
/// assert_eq!(remainder.to_ascii_lowercase().trim(), "then more");
/// ```
pub fn ignore_eol_comment<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, ()> {
    value(
        (), // Output is thrown away.
        opt(terminated(
            pair(char(';'), is_not("\r\n")),
            tuple((multispace0, opt(ignore_eol_comment))),
        )),
    )(input.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_line() {
        let input = "; nothing";
        let (remainder, comment) = ignore_eol_comment(input).unwrap();
        assert_eq!(comment, ());
        assert!(remainder.is_empty());
    }

    #[test]
    fn precedes_text() {
        let input = "; nothing\n\r\tlast line";
        let (remainder, comment) = ignore_eol_comment(input).unwrap();
        assert_eq!(comment, ());
        assert_eq!(remainder.to_ascii_lowercase(), "last line");
    }
}
