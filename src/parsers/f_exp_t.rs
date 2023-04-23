//! Provides parsers for f-exps.

use crate::parsers::parse_f_exp;
use crate::parsers::prefix_expr;
use crate::types::FExpT;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

/// Parses an f-exp-t.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_exp, parse_f_exp_t};
/// # use pddl::types::{BinaryOp, FExp, FExpT, FHead, FunctionSymbol, MultiOp, Term, Variable};
/// assert_eq!(parse_f_exp_t("#t"), Ok(("", FExpT::Now)));
///
/// assert_eq!(parse_f_exp_t("(* (fuel ?tank) #t)"), Ok(("",
///     FExpT::new_scaled(
///         FExp::new_f_head(
///             FHead::new_with_terms(
///                 FunctionSymbol::from_str("fuel"),
///                 [Term::Variable(Variable::from_str("tank"))]
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_f_exp_t("(* #t (fuel ?tank))"), Ok(("",
///     FExpT::new_scaled(
///         FExp::new_f_head(
///             FHead::new_with_terms(
///                 FunctionSymbol::from_str("fuel"),
///                 [Term::Variable(Variable::from_str("tank"))]
///             )
///         )
///     )
/// )));
///```
pub fn parse_f_exp_t(input: &str) -> IResult<&str, FExpT> {
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

    alt((scaled, now))(input)
}
