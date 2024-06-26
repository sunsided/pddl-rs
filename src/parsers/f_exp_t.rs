//! Provides parsers for f-exps.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated, tuple};

use crate::parsers::prefix_expr;
use crate::parsers::{parse_f_exp, ParseResult, Span};
use crate::types::FExpT;

/// Parses an f-exp-t.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_exp, parse_f_exp_t, preamble::*};
/// # use pddl::{BinaryOp, FExp, FExpT, FHead, FunctionSymbol, MultiOp, Term, Variable};
/// assert!(parse_f_exp_t("#t").is_value(FExpT::Now));
///
/// assert!(parse_f_exp_t("(* (fuel ?tank) #t)").is_value(
///     FExpT::new_scaled(
///         FExp::new_function(
///             FHead::new_with_terms(
///                 FunctionSymbol::from_str("fuel"),
///                 [Term::Variable(Variable::from_str("tank"))]
///             )
///         )
///     )
/// ));
///
/// assert!(parse_f_exp_t("(* #t (fuel ?tank))").is_value(
///     FExpT::new_scaled(
///         FExp::new_function(
///             FHead::new_with_terms(
///                 FunctionSymbol::from_str("fuel"),
///                 [Term::Variable(Variable::from_str("tank"))]
///             )
///         )
///     )
/// ));
///```
pub fn parse_f_exp_t<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FExpT> {
    let now = map(tag("#t"), |_| FExpT::new());
    let scaled = map(
        prefix_expr(
            "*",
            alt((
                preceded(tuple((tag("#t"), multispace1)), parse_f_exp),
                terminated(parse_f_exp, tuple((multispace1, tag("#t")))),
            )),
        ),
        FExpT::new_scaled,
    );

    alt((scaled, now))(input.into())
}

impl crate::parsers::Parser for FExpT {
    type Item = FExpT;

    /// See [`parse_f_exp_t`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_exp_t(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{FExp, FExpT, FHead, FunctionSymbol, Parser, Term, Variable};

    #[test]
    fn test_parse() {
        assert!(FExpT::parse("#t").is_value(FExpT::Now));

        assert!(
            FExpT::parse("(* (fuel ?tank) #t)").is_value(FExpT::new_scaled(FExp::new_function(
                FHead::new_with_terms(
                    FunctionSymbol::from_str("fuel"),
                    [Term::Variable(Variable::from_str("tank"))]
                )
            )))
        );

        assert!(
            FExpT::parse("(* #t (fuel ?tank))").is_value(FExpT::new_scaled(FExp::new_function(
                FHead::new_with_terms(
                    FunctionSymbol::from_str("fuel"),
                    [Term::Variable(Variable::from_str("tank"))]
                )
            )))
        );
    }
}
