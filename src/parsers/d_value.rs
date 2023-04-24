//! Provides parsers for d-values.

use crate::parsers::parse_f_exp;
use crate::parsers::parse_number;
use crate::types::DurationValue;
use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

/// Parses a d-value.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_d_value;
/// # use pddl::{BinaryOp, DurationValue, FExp, FHead, FunctionSymbol, MultiOp};
/// assert_eq!(parse_d_value("1.23"), Ok(("",
///     DurationValue::new_number(1.23)
/// )));
///
/// assert_eq!(parse_d_value("fun-sym"), Ok(("",
///     DurationValue::new_f_exp(
///         FExp::new_function(FHead::Simple("fun-sym".into()))
///     )
/// )));
///```
pub fn parse_d_value(input: &str) -> IResult<&str, DurationValue> {
    let number = map(parse_number, DurationValue::new_number);

    // :numeric-fluents
    let f_exp = map(parse_f_exp, DurationValue::new_f_exp);

    alt((number, f_exp))(input)
}

impl<'a> crate::parsers::Parser<'a> for DurationValue<'a> {
    type Item = DurationValue<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_d_value(input)
    }
}
