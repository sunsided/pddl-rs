//! Provides parsers for f-heads.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::{parens, space_separated_list0, ParseResult, Span};
use crate::parsers::{parse_function_symbol, parse_term};
use crate::types::FunctionHead;

/// Parses an f-head.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_head, preamble::*};
/// # use pddl::{FunctionTerm, Variable, FunctionSymbol, Term, FunctionHead};
/// assert!(parse_f_head("fun-sym").is_value(
///     FunctionHead::new(FunctionSymbol::new_string("fun-sym"))
/// ));
///
/// assert!(parse_f_head("(fun-sym)").is_value(
///     FunctionHead::new(FunctionSymbol::new_string("fun-sym"))
/// ));
///
/// assert!(parse_f_head("(fun-sym term)").is_value(
///     FunctionHead::new_with_terms(FunctionSymbol::new_string("fun-sym"), [
///         Term::Name("term".into())
///     ])
/// ));
///```
pub fn parse_f_head<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FunctionHead> {
    let simple = map(parse_function_symbol, FunctionHead::new);
    let simple_parens = map(parens(parse_function_symbol), FunctionHead::new);
    let with_terms = map(
        parens((
            parse_function_symbol,
            preceded(multispace1, space_separated_list0(parse_term)),
        )),
        |(symbol, terms)| FunctionHead::new_with_terms(symbol, terms),
    );

    alt((simple, simple_parens, with_terms)).parse(input.into())
}

impl crate::parsers::Parser for FunctionHead {
    type Item = FunctionHead;

    /// See [`parse_f_head`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_head(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{FunctionHead, FunctionSymbol, Parser, Term};

    #[test]
    fn test_parse() {
        assert!(FunctionHead::parse("fun-sym")
            .is_value(FunctionHead::new(FunctionSymbol::new_string("fun-sym"))));

        assert!(FunctionHead::parse("(fun-sym)")
            .is_value(FunctionHead::new(FunctionSymbol::new_string("fun-sym"))));

        assert!(
            FunctionHead::parse("(fun-sym term)").is_value(FunctionHead::new_with_terms(
                FunctionSymbol::new_string("fun-sym"),
                [Term::Name("term".into())]
            ))
        );
    }
}
