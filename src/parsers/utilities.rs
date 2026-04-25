//! Utility parsers.

use crate::parsers::{ParseError, ParseResult, Span};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{char, multispace1};
use nom::combinator::{recognize, value};
use nom::multi::{many0, separated_list0, separated_list1};
use nom::sequence::{delimited, pair, preceded};
use nom::Parser;

/// Recognizes a semicolon comment up to, but not including, the line ending.
/// Accepts empty comments such as `;` followed immediately by `\n` or `\r\n`.
fn eol_comment<'a>(input: Span<'a>) -> ParseResult<'a, Span<'a>> {
    recognize(pair(
        char(';'),
        take_while(|c: char| c != '\n' && c != '\r'),
    ))
    .parse(input)
}

/// Skips all leading whitespace and semicolon comments.
/// Handles multiple comment lines, including empty ones (`;\n`).
fn skip_whitespace_and_comments<'a>(input: Span<'a>) -> ParseResult<'a, ()> {
    value((), many0(alt((multispace1, eol_comment)))).parse(input)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes a leading `(name` and trailing `)`, returning the output of `inner`.
#[allow(clippy::needless_lifetimes)]
pub fn prefix_expr<'a, P, O>(
    name: &'a str,
    inner: P,
) -> impl Parser<Span<'a>, Output = O, Error = ParseError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    delimited(preceded(ws(tag("(")), tag(name)), ws(inner), ws(tag(")")))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes leading whitespace,
/// returning the output of `inner`.
///
/// This parser also suppresses line comments.
pub fn ws<'a, P, O>(inner: P) -> impl Parser<Span<'a>, Output = O, Error = ParseError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    preceded(skip_whitespace_and_comments, inner)
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes leading
/// and trailing whitespace, returning the output of `inner`.
///
/// This parser also suppresses line comments.
pub fn ws2<'a, P, O>(inner: P) -> impl Parser<Span<'a>, Output = O, Error = ParseError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    delimited(
        skip_whitespace_and_comments,
        inner,
        skip_whitespace_and_comments,
    )
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
#[allow(dead_code)]
pub fn space_separated_list0<'a, P, O>(
    inner: P,
) -> impl Parser<Span<'a>, Output = Vec<O>, Error = ParseError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    ws(separated_list0(
        multispace1,
        preceded(skip_whitespace_and_comments, inner),
    ))
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
pub fn space_separated_list1<'a, P, O>(
    inner: P,
) -> impl Parser<Span<'a>, Output = Vec<O>, Error = ParseError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    ws(separated_list1(
        multispace1,
        preceded(skip_whitespace_and_comments, inner),
    ))
}

/// A combinator that takes a parser `inner` and produces a parser that consumes
/// surrounding parentheses, returning the outputs of `inner`.
pub fn parens<'a, P, O>(inner: P) -> impl Parser<Span<'a>, Output = O, Error = ParseError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    preceded(
        skip_whitespace_and_comments,
        delimited(char('('), ws(inner), char(')')),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{parse_name, Match};
    use crate::Name;
    use nom::multi::separated_list1;

    #[test]
    fn parens_works() {
        let input = "(content)";
        let mut parser = parens(parse_name);
        assert!(parser.parse(Span::new(input)).is_exactly("content"));
    }

    #[test]
    fn definition_section_works() {
        let input = "(either x y)";
        let inner_parser = separated_list1(tag(" "), parse_name);
        let mut parser = prefix_expr("either", inner_parser);
        assert!(parser
            .parse(Span::new(input))
            .is_exactly(vec![Name::from("x"), Name::from("y")]));
    }

    #[test]
    fn space_separated_list0_works() {
        let mut parser = space_separated_list0(parse_name);
        assert!(parser
            .parse(Span::new("x y"))
            .is_exactly(vec![Name::from("x"), Name::from("y")]));
        assert!(parser
            .parse(Span::new("x"))
            .is_exactly(vec![Name::from("x")]));
        assert!(parser.parse(Span::new("")).is_exactly(vec![]));
    }

    #[test]
    fn space_separated_list1_works() {
        let mut parser = space_separated_list1(parse_name);
        assert!(parser
            .parse(Span::new("x y"))
            .is_exactly(vec![Name::from("x"), Name::from("y")]));
        assert!(parser
            .parse(Span::new("x"))
            .is_exactly(vec![Name::from("x")]));
        assert!(parser.parse(Span::new("")).is_err());
    }

    #[test]
    fn leading_empty_comment() {
        let input = ";\ncontent";
        let mut parser = ws(tag("content"));
        let (remainder, _) = parser.parse(Span::new(input)).unwrap();
        assert!(remainder.fragment().is_empty());
    }

    #[test]
    fn multiple_leading_comments() {
        let input = "; comment 1\n; comment 2\n; comment 3\ncontent";
        let mut parser = ws(tag("content"));
        let (remainder, _) = parser.parse(Span::new(input)).unwrap();
        assert!(remainder.fragment().is_empty());
    }

    #[test]
    fn mixed_comments_and_whitespace() {
        let input = "\n  ; comment\n\n  ; another\n\t\ncontent";
        let mut parser = ws(tag("content"));
        let (remainder, _) = parser.parse(Span::new(input)).unwrap();
        assert!(remainder.fragment().is_empty());
    }

    #[test]
    fn domain_with_leading_comments() {
        let input = r#"; This is a comment
; Another comment
(define (domain test)
    (:requirements :strips)
    (:predicates (p))
)
"#;
        let (_, domain) = crate::parsers::parse_domain(Span::new(input)).unwrap();
        assert_eq!(domain.name(), &crate::Name::new("test"));
    }
}
