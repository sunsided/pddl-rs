//! Provides the [`empty_or`] parser combinator.

use crate::parsers::{ParseResult, Span};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::ParseError;

/// Parser combinator that takes a parser `inner` and produces a parser that
/// consumes `()` and returns [`None`] or the result of `inner` and produces [`Some(O)`](Some).
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_variable, Span, preamble::*};
/// # use pddl::parsers::empty_or;
/// # use pddl::Variable;
/// let mut parser = empty_or(parse_variable);
/// assert!(parser(Span::new("()")).is_value(None));
/// assert!(parser(Span::new("?abc")).is_value(Some(Variable::from("abc"))));
/// ```
#[allow(dead_code)]
pub fn empty_or<'a, F, O, E: ParseError<Span<'a>>>(
    inner: F,
) -> impl FnMut(Span<'a>) -> ParseResult<'a, Option<O>, E>
where
    F: FnMut(Span<'a>) -> ParseResult<'a, O, E>,
{
    let empty_parser = map(tag("()"), |_: Span| None);
    let inner_parser = map(inner, |o: O| Some(o));

    alt((empty_parser, inner_parser))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::Match;
    use nom::character::complete::alpha1;

    #[test]
    fn empty_or_works() {
        let mut parser = empty_or(alpha1::<Span<'static>, crate::parsers::ParseError>);
        assert!(parser(Span::new("()")).is_exactly(None));
        assert!(parser(Span::new("abc")).is_exactly(Some("abc")));
    }
}
