//! Provides parsers for initial state list elements.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

use crate::parsers::{
    literal, parse_basic_function_term, parse_name, parse_number, prefix_expr, ParseResult, Span,
};
use crate::types::InitElement;

/// Parses initial state list elements.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_init_el, preamble::*};
/// # use pddl::{AtomicFormula, BasicFunctionTerm, InitElement, Name, NameLiteral, Number, Predicate};
/// assert!(parse_init_el("(train-not-in-use train1)").is_value(
///     InitElement::new_literal(
///         NameLiteral::new(
///             AtomicFormula::new_predicate(
///                 "train-not-in-use".into(),
///                 ["train1".into()]
///             )
///         )
///     )
/// ));
///
/// assert!(parse_init_el("(at 10 (train-not-in-use train2))").is_value(
///     InitElement::new_at(
///         Number::from(10),
///         NameLiteral::new(
///             AtomicFormula::new_predicate(
///                 "train-not-in-use".into(),
///                 ["train2".into()]
///             )
///         )
///     )
/// ));
///
/// assert!(parse_init_el("(= (battery rover) 100)").is_value(
///     InitElement::new_is_value(
///         BasicFunctionTerm::new("battery".into(), ["rover".into()]),
///         Number::from(100)
///     )
/// ));
///
/// assert!(parse_init_el("(= (location rover) base)").is_value(
///     InitElement::new_is_object(
///         BasicFunctionTerm::new("location".into(), ["rover".into()]),
///         Name::from("base")
///     )
/// ));
/// ```
pub fn parse_init_el<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, InitElement> {
    let literal_ = map(literal(parse_name), InitElement::new_literal);

    // :timed-initial-literals
    let at = map(
        prefix_expr(
            "at",
            tuple((parse_number, preceded(multispace1, literal(parse_name)))),
        ),
        |(time, name)| InitElement::new_at(time, name),
    );

    // :numeric-fluents
    let is_numeric = map(
        prefix_expr(
            "=",
            tuple((
                parse_basic_function_term,
                preceded(multispace1, parse_number),
            )),
        ),
        |(fun, value)| InitElement::new_is_value(fun, value),
    );

    // :object-fluents
    let is_object = map(
        prefix_expr(
            "=",
            tuple((parse_basic_function_term, preceded(multispace1, parse_name))),
        ),
        |(fun, name)| InitElement::new_is_object(fun, name),
    );

    alt((literal_, at, is_numeric, is_object))(input.into())
}

impl crate::parsers::Parser for InitElement {
    type Item = InitElement;

    /// See [`parse_init_el`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_init_el(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::UnwrapValue;
    use crate::{AtomicFormula, BasicFunctionTerm, InitElement, Name, NameLiteral, Number, Parser};

    #[test]
    fn test_parse() {
        assert!(InitElement::parse("(train-not-in-use train1)").is_value(
            InitElement::new_literal(NameLiteral::new(AtomicFormula::new_predicate(
                "train-not-in-use".into(),
                ["train1".into()]
            )))
        ));

        assert!(
            InitElement::parse("(at 10 (train-not-in-use train2))").is_value(InitElement::new_at(
                Number::from(10),
                NameLiteral::new(AtomicFormula::new_predicate(
                    "train-not-in-use".into(),
                    ["train2".into()]
                ))
            ))
        );

        assert!(
            InitElement::parse("(= (battery rover) 100)").is_value(InitElement::new_is_value(
                BasicFunctionTerm::new("battery".into(), ["rover".into()]),
                Number::from(100)
            ))
        );

        assert!(InitElement::parse("(= (location rover) base)").is_value(
            InitElement::new_is_object(
                BasicFunctionTerm::new("location".into(), ["rover".into()]),
                Name::from("base")
            )
        ));
    }
}
