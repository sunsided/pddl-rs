//! Provides the [`empty_or`] parser combinator.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::ParseError;
use nom::Parser;

use crate::parsers::{ParseError as CrateParseError, ParseResult, Span};

/// Parser combinator that takes a parser `inner` and produces a parser that
/// consumes `()` and returns [`None`] or the result of `inner` and produces [`Some(O)`](Some).
///
/// ## Example
/// ```
/// # use nom::Parser;
/// # use pddl::parsers::{parse_variable, Span, preamble::*};
/// # use pddl::parsers::empty_or;
/// # use pddl::Variable;
/// let mut parser = empty_or(parse_variable);
/// assert!(parser.parse(Span::new("()")).is_value(None));
/// assert!(parser.parse(Span::new("?abc")).is_value(Some(Variable::from("abc"))));
/// ```
#[allow(dead_code)]
pub fn empty_or<'a, P, O, E: ParseError<Span<'a>>>(
    inner: P,
) -> impl Parser<Span<'a>, Output = Option<O>, Error = E>
where
    P: Parser<Span<'a>, Output = O, Error = E>,
{
    let empty_parser = map(tag("()"), |_: Span| None);
    let inner_parser = map(inner, |o: O| Some(o));

    alt((empty_parser, inner_parser))
}

#[cfg(test)]
mod tests {
    use nom::character::complete::alpha1;
    use nom::Parser;

    use crate::parsers::Match;
    use crate::parsers::ParseError;

    use super::*;

    #[test]
    fn empty_or_works() {
        let mut parser = empty_or(alpha1::<Span<'static>, ParseError<'static>>);
        assert!(parser.parse(Span::new("()")).is_exactly(None));
        assert!(parser.parse(Span::new("abc")).is_exactly(Some("abc")));
    }
}
