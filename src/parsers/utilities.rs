//! Utility parsers.

use crate::parsers::{ignore_eol_comment, ParseError, ParseResult, Span};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::Parser;

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
    preceded(preceded(multispace0, ignore_eol_comment), inner)
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
        preceded(multispace0, ignore_eol_comment),
        inner,
        preceded(multispace0, ignore_eol_comment),
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
        preceded(ignore_eol_comment, inner),
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
        preceded(ignore_eol_comment, inner),
    ))
}

/// A combinator that takes a parser `inner` and produces a parser that consumes
/// surrounding parentheses, returning the outputs of `inner`.
pub fn parens<'a, P, O>(inner: P) -> impl Parser<Span<'a>, Output = O, Error = ParseError<'a>>
where
    P: Parser<Span<'a>, Output = O, Error = ParseError<'a>>,
{
    preceded(
        ignore_eol_comment,
        delimited(char('('), ws(inner), char(')')),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::{parse_name, Match};
    use crate::Name;
    use nom::multi::separated_list1;
    use nom::Parser;

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
}
