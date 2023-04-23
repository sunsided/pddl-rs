//! Provides parsers for d-values.

use crate::parsers::domain::parse_f_exp;
use crate::parsers::utility::parse_number;
use crate::types::DValue;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parses a d-value.
///
/// ## Example
/// ```
/// # use pddl::parsers::domain::parse_d_value;
/// # use pddl::types::{BinaryOp, DValue, FExp, FHead, FunctionSymbol, MultiOp};
/// assert_eq!(parse_d_value("1.23"), Ok(("",
///     DValue::new_number(1.23)
/// )));
///
/// assert_eq!(parse_d_value("fun-sym"), Ok(("",
///     DValue::new_f_exp(
///         FExp::new_f_head(FHead::Simple("fun-sym".into()))
///     )
/// )));
///```
pub fn parse_d_value(input: &str) -> IResult<&str, DValue> {
    let number = map(parse_number, DValue::new_number);

    // :numeric-fluents
    let f_exp = map(parse_f_exp, DValue::new_f_exp);

    alt((number, f_exp))(input)
}
