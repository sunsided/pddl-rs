//! Provides parsers for basic function terms.

use crate::parsers::{
    parens, parse_function_symbol, parse_name, space_separated_list0, ws, ParseResult, Span,
};
use crate::types::BasicFunctionTerm;
use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::tuple;

/// Parses a basic function term.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_basic_function_term, UnwrapValue, Span};
/// # use pddl::{BasicFunctionTerm, Term};
/// assert!(parse_basic_function_term(Span::new("abcde")).is_value(
///     BasicFunctionTerm::new("abcde".into(), [])
/// ));
///
/// assert!(parse_basic_function_term(Span::new("(abcde)")).is_value(
///     BasicFunctionTerm::new("abcde".into(), [])
/// ));
///
/// assert!(parse_basic_function_term(Span::new("(abcde x y z)")).is_value(
///     BasicFunctionTerm::new("abcde".into(), ["x".into(), "y".into(), "z".into()])
/// ));
///```
pub fn parse_basic_function_term<'a, T: Into<Span<'a>>>(
    input: T,
) -> ParseResult<'a, BasicFunctionTerm> {
    let direct = map(parse_function_symbol, |s| BasicFunctionTerm::new(s, []));
    let named = map(
        parens(tuple((
            parse_function_symbol,
            ws(space_separated_list0(parse_name)),
        ))),
        |(s, ns)| BasicFunctionTerm::new(s, ns),
    );
    alt((direct, named))(input.into())
}

impl crate::parsers::Parser for BasicFunctionTerm {
    type Item = BasicFunctionTerm;

    /// Parses a basic function term.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{BasicFunctionTerm, Parser};
    /// let (_, value) = BasicFunctionTerm::parse("abcde").unwrap();
    /// assert_eq!(value,  BasicFunctionTerm::new("abcde".into(), []));
    ///
    /// let (_, value) = BasicFunctionTerm::parse("(abcde)").unwrap();
    /// assert_eq!(value,  BasicFunctionTerm::new("abcde".into(), []));
    ///
    /// let (_, value) = BasicFunctionTerm::parse("(abcde x y z)").unwrap();
    /// assert_eq!(value,  BasicFunctionTerm::new("abcde".into(), [
    ///     "x".into(), "y".into(), "z".into()]
    /// ));
    /// ```
    ///
    /// ## See also
    /// See [`parse_basic_function_term`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_basic_function_term(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BasicFunctionTerm, Parser};

    #[test]
    fn test_parse() {
        let (_, value) = BasicFunctionTerm::parse("abcde").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));

        let (_, value) = BasicFunctionTerm::parse("(abcde)").unwrap();
        assert_eq!(value, BasicFunctionTerm::new("abcde".into(), []));

        let (_, value) = BasicFunctionTerm::parse("(abcde x y z)").unwrap();
        assert_eq!(
            value,
            BasicFunctionTerm::new("abcde".into(), ["x".into(), "y".into(), "z".into()])
        );
    }
}
