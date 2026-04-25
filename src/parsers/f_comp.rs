//! Provides parsers for f-comps.

use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::{parens, ParseResult, Span};
use crate::parsers::{parse_binary_comp, parse_f_exp};
use crate::types::FluentComparison;

/// Parses an f-comp.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_f_comp, preamble::*};
/// # use pddl::{FunctionTerm, Variable, FunctionSymbol, Term, FluentComparison, BinaryComparison, FluentExpression, BinaryOp};
/// assert!(parse_f_comp("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(
///     FluentComparison::new(
///         BinaryComparison::Equal,
///         FluentExpression::new_binary_op(
///             BinaryOp::Addition,
///             FluentExpression::new_number(1.23),
///             FluentExpression::new_number(2.34),
///         ),
///         FluentExpression::new_binary_op(
///             BinaryOp::Addition,
///             FluentExpression::new_number(1.23),
///             FluentExpression::new_number(2.34),
///         )
///     )
/// ));
///```
pub fn parse_f_comp<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, FluentComparison> {
    map(
        parens((
            parse_binary_comp,
            preceded(multispace1, parse_f_exp),
            preceded(multispace1, parse_f_exp),
        )),
        FluentComparison::from,
    )
    .parse(input.into())
}

impl crate::parsers::Parser for FluentComparison {
    type Item = FluentComparison;

    /// See [`parse_f_comp`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_f_comp(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{BinaryComparison, BinaryOp, FluentComparison, FluentExpression, Parser};

    #[test]
    fn test_parse() {
        assert!(
            FluentComparison::parse("(= (+ 1.23 2.34) (+ 1.23 2.34))").is_value(FluentComparison::new(
                BinaryComparison::Equal,
                FluentExpression::new_binary_op(
                    BinaryOp::Addition,
                    FluentExpression::new_number(1.23),
                    FluentExpression::new_number(2.34),
                ),
                FluentExpression::new_binary_op(
                    BinaryOp::Addition,
                    FluentExpression::new_number(1.23),
                    FluentExpression::new_number(2.34),
                )
            ))
        );
    }
}
