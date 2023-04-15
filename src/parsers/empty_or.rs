use std::convert::Infallible;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map_res;
use nom::IResult;

/// Parser combinator that takes a parser `inner` and produces a parser that
/// consumes `()` and returns [`None`] or the result of `inner` and produces [`Some(O)`](Some).
///
/// ## Example
/// ```
/// # use pddl::parsers::{empty_or, parse_variable};
/// # use pddl::types::Variable;
/// let mut parser = empty_or(parse_variable);
/// assert_eq!(parser("()"), Ok(("", None)));
/// assert_eq!(parser("?abc"), Ok(("", Some(Variable::from("abc")))));
/// ```
#[allow(dead_code)]
pub fn empty_or<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, Option<O>>
    where
        F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    let empty_parser = map_res(tag("()"), |_: &str| Ok::<Option<O>, Infallible>(None));
    let inner_parser = map_res(inner, |o: O| Ok::<Option<O>, Infallible>(Some(o)));

    alt((empty_parser, inner_parser))
}


#[cfg(test)]
mod tests {
    use nom::character::complete::alpha1;
    use super::*;

    #[test]
    fn empty_or_works() {
        let mut parser = empty_or(alpha1);
        assert_eq!(parser("()"), Ok(("", None)));
        assert_eq!(parser("abc"), Ok(("", Some("abc"))));
    }
}
