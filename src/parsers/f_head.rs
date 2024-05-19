//! Provides parsers for f-heads.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

use crate::parsers::{parens, space_separated_list0, ParseResult, Span};
use crate::parsers::{parse_function_symbol, parse_term};
use crate::types::FHead;

/// Parses an f-head.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_head, preamble::*};
/// # use pddl::{FunctionTerm, Variable, FunctionSymbol, Term, FHead};
/// assert!(parse_f_head("fun-sym").is_value(
///     FHead::new(FunctionSymbol::from_str("fun-sym"))
/// ));
///
/// assert!(parse_f_head("(fun-sym)").is_value(
///     FHead::new(FunctionSymbol::from_str("fun-sym"))
/// ));
///
/// assert!(parse_f_head("(fun-sym term)").is_value(
///     FHead::new_with_terms(FunctionSymbol::from_str("fun-sym"), [
///         Term::Name("term".into())
///     ])
/// ));
///```
pub fn parse_f_head<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FHead> {
    let simple = map(parse_function_symbol, FHead::new);
    let simple_parens = map(parens(parse_function_symbol), FHead::new);
    let with_terms = map(
        parens(tuple((
            parse_function_symbol,
            preceded(multispace1, space_separated_list0(parse_term)),
        ))),
        |(symbol, terms)| FHead::new_with_terms(symbol, terms),
    );

    alt((simple, simple_parens, with_terms))(input.into())
}

impl crate::parsers::Parser for FHead {
    type Item = FHead;

    /// See [`parse_f_head`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_head(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{FHead, FunctionSymbol, Parser, Term};

    #[test]
    fn test_parse() {
        assert!(FHead::parse("fun-sym").is_value(FHead::new(FunctionSymbol::from_str("fun-sym"))));

        assert!(FHead::parse("(fun-sym)").is_value(FHead::new(FunctionSymbol::from_str("fun-sym"))));

        assert!(
            FHead::parse("(fun-sym term)").is_value(FHead::new_with_terms(
                FunctionSymbol::from_str("fun-sym"),
                [Term::Name("term".into())]
            ))
        );
    }
}
