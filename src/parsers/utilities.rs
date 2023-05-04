//! Utility parsers.

use crate::parsers::{ParseResult, Span};
use nom::bytes::complete::tag;
use nom::character::complete::{char, multispace0, multispace1};
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded};

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a leading `(name` and trailing `)`, returning the output of `inner`.
#[allow(clippy::needless_lifetimes)]
pub fn prefix_expr<'a, F, O>(name: &'a str, inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, O>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    delimited(preceded(tag("("), tag(name)), ws(inner), tag(")"))
}

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
pub fn ws<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, O>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
#[allow(dead_code)]
pub fn space_separated_list0<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, Vec<O>>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    ws(separated_list0(multispace1, inner))
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
pub fn space_separated_list1<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, Vec<O>>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    ws(separated_list1(multispace1, inner))
}

/// A combinator that takes a parser `inner` and produces a parser that consumes
/// surrounding parentheses, returning the outputs of `inner`.
pub fn parens<'a, F, O>(inner: F) -> impl FnMut(Span<'a>) -> ParseResult<'a, O>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O>,
{
    delimited(char('('), ws(inner), char(')'))
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
        assert!(parser(Span::new(input)).is_exactly("content"));
    }

    #[test]
    fn definition_section_works() {
        let input = "(either x y)";
        let inner_parser = separated_list1(tag(" "), parse_name);
        let mut parser = prefix_expr("either", inner_parser);
        assert!(parser(Span::new(input)).is_exactly(vec![Name::from("x"), Name::from("y")]));
    }

    #[test]
    fn space_separated_list0_works() {
        let mut parser = space_separated_list0(parse_name);
        assert!(parser(Span::new("x y")).is_exactly(vec![Name::from("x"), Name::from("y")]));
        assert!(parser(Span::new("x")).is_exactly(vec![Name::from("x")]));
        assert!(parser(Span::new("")).is_exactly(vec![]));
    }

    #[test]
    fn space_separated_list1_works() {
        let mut parser = space_separated_list1(parse_name);
        assert!(parser(Span::new("x y")).is_exactly(vec![Name::from("x"), Name::from("y")]));
        assert!(parser(Span::new("x")).is_exactly(vec![Name::from("x")]));
        assert!(parser(Span::new("")).is_err());
    }
}
