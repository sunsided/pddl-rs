//! Provides parsers for f-exps.

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use nom::Parser;

use crate::parsers::prefix_expr;
use crate::parsers::{parse_f_exp, ParseResult, Span};
use crate::types::TimedFluentExpression;

/// Parses an f-exp-t.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_exp, parse_f_exp_t, preamble::*};
/// # use pddl::{BinaryOp, FluentExpression, TimedFluentExpression, FunctionHead, FunctionSymbol, MultiOp, Term, Variable};
/// assert!(parse_f_exp_t("#t").is_value(TimedFluentExpression::Now));
///
/// assert!(parse_f_exp_t("(* (fuel ?tank) #t)").is_value(
///     TimedFluentExpression::new_scaled(
///         FluentExpression::new_function(
///             FunctionHead::new_with_terms(
///                 FunctionSymbol::new_string("fuel"),
///                 [Term::Variable(Variable::new_string("tank"))]
///             )
///         )
///     )
/// ));
///
/// assert!(parse_f_exp_t("(* #t (fuel ?tank))").is_value(
///     TimedFluentExpression::new_scaled(
///         FluentExpression::new_function(
///             FunctionHead::new_with_terms(
///                 FunctionSymbol::new_string("fuel"),
///                 [Term::Variable(Variable::new_string("tank"))]
///             )
///         )
///     )
/// ));
///```
pub fn parse_f_exp_t<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, TimedFluentExpression> {
    let now = map(tag("#t"), |_| TimedFluentExpression::new());
    let scaled = map(
        prefix_expr(
            "*",
            alt((
                preceded((tag("#t"), multispace1), parse_f_exp),
                terminated(parse_f_exp, (multispace1, tag("#t"))),
            )),
        ),
        TimedFluentExpression::new_scaled,
    );

    alt((scaled, now)).parse(input.into())
}

impl crate::parsers::Parser for TimedFluentExpression {
    type Item = TimedFluentExpression;

    /// See [`parse_f_exp_t`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_exp_t(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{FluentExpression, TimedFluentExpression, FunctionHead, FunctionSymbol, Parser, Term, Variable};

    #[test]
    fn test_parse() {
        assert!(TimedFluentExpression::parse("#t").is_value(TimedFluentExpression::Now));

        assert!(
            TimedFluentExpression::parse("(* (fuel ?tank) #t)").is_value(TimedFluentExpression::new_scaled(FluentExpression::new_function(
                FunctionHead::new_with_terms(
                    FunctionSymbol::new_string("fuel"),
                    [Term::Variable(Variable::new_string("tank"))]
                )
            )))
        );

        assert!(
            TimedFluentExpression::parse("(* #t (fuel ?tank))").is_value(TimedFluentExpression::new_scaled(FluentExpression::new_function(
                FunctionHead::new_with_terms(
                    FunctionSymbol::new_string("fuel"),
                    [Term::Variable(Variable::new_string("tank"))]
                )
            )))
        );
    }
}
