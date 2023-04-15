mod function_symbol;
mod name;
mod number;
mod predicate;
mod primitive_type;
mod requirements;
mod r#type;
mod variable;
mod term;
mod function_term;

use std::convert::Infallible;
use nom::branch::alt;
pub use function_symbol::parse_function_symbol;
pub use name::parse_name;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::{map_res};
use nom::error::ParseError;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::{delimited, preceded};
use nom::IResult;
pub use number::parse_number;
pub use predicate::parse_predicate;
pub use primitive_type::parse_primitive_type;
pub use r#type::parse_type;
pub use requirements::{parse_require_def, parse_require_key};
pub use variable::parse_variable;
pub use term::parse_term;
pub use function_term::parse_function_term;

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

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a leading `(name` and trailing `)`, returning the output of `inner`.
#[allow(clippy::needless_lifetimes)]
fn definition_section<'a, F, O>(
    name: &'a str,
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    delimited(preceded(tag("("), tag(name)), ws(inner), tag(")"))
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
#[allow(dead_code)]
fn space_separated_list0<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    ws(separated_list0(multispace1, inner))
}

/// A combinator that takes a parser `inner` and produces a parser that also
/// consumes a whitespace separated list, returning the outputs of `inner`.
fn space_separated_list1<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<O>>
where
    F: FnMut(&'a str) -> IResult<&'a str, O>,
{
    ws(separated_list1(multispace1, inner))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::character::complete::alpha1;
    use nom::multi::separated_list1;

    #[test]
    fn empty_or_works() {
        let mut parser = empty_or(alpha1);
        assert_eq!(parser("()"), Ok(("", None)));
        assert_eq!(parser("abc"), Ok(("", Some("abc"))));
    }

    #[test]
    fn definition_section_works() {
        let input = "(either x y)";
        let inner_parser = separated_list1(tag(" "), alpha1);
        let mut parser = definition_section("either", inner_parser);
        assert_eq!(parser(input), Ok(("", vec!["x", "y"])));
    }

    #[test]
    fn space_separated_list0_works() {
        let mut parser = space_separated_list0(alpha1);
        assert_eq!(parser("x y"), Ok(("", vec!["x", "y"])));
        assert_eq!(parser("x"), Ok(("", vec!["x"])));
        assert_eq!(parser(""), Ok(("", vec![])));
    }

    #[test]
    fn space_separated_list1_works() {
        let mut parser = space_separated_list1(alpha1);
        assert_eq!(parser("x y"), Ok(("", vec!["x", "y"])));
        assert_eq!(parser("x"), Ok(("", vec!["x"])));
        assert!(parser("").is_err());
    }
}
