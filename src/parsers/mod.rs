mod function_symbol;
mod name;
mod number;
mod predicate;
mod requirements;
mod variable;

pub use function_symbol::parse_function_symbol;
pub use name::parse_name;
use nom::character::complete::multispace0;
use nom::error::ParseError;
use nom::sequence::delimited;
use nom::IResult;
pub use number::parse_number;
pub use predicate::parse_predicate;
pub use requirements::{parse_require_def, parse_require_key};
pub use variable::parse_variable;

/// A combinator that takes a parser `inner` and produces a parser that also consumes both leading and
/// trailing whitespace, returning the output of `inner`.
fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
