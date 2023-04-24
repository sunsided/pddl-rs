//! Provides parsers for initial state list elements.

use crate::parsers::{literal, parse_basic_function_term, parse_name, parse_number, prefix_expr};
use crate::types::InitElement;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parses initial state list elements.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_init_el;
/// # use pddl::{AtomicFormula, BasicFunctionTerm, InitElement, Name, NameLiteral, Number, Predicate};
/// assert_eq!(parse_init_el("(train-not-in-use train1)"), Ok(("",
///     InitElement::new_literal(
///         NameLiteral::new(
///             AtomicFormula::new_predicate(
///                 "train-not-in-use".into(),
///                 ["train1".into()]
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_init_el("(at 10 (train-not-in-use train2))"), Ok(("",
///     InitElement::new_at(
///         Number::from(10),
///         NameLiteral::new(
///             AtomicFormula::new_predicate(
///                 "train-not-in-use".into(),
///                 ["train2".into()]
///             )
///         )
///     )
/// )));
///
/// assert_eq!(parse_init_el("(= (battery rover) 100)"), Ok(("",
///     InitElement::new_is_value(
///         BasicFunctionTerm::new("battery".into(), ["rover".into()]),
///         Number::from(100)
///     )
/// )));
///
/// assert_eq!(parse_init_el("(= (location rover) base)"), Ok(("",
///     InitElement::new_is_object(
///         BasicFunctionTerm::new("location".into(), ["rover".into()]),
///         Name::from("base")
///     )
/// )));
/// ```
pub fn parse_init_el(input: &str) -> IResult<&str, InitElement> {
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

    alt((literal_, at, is_numeric, is_object))(input)
}

impl<'a> crate::parsers::Parser<'a> for InitElement<'a> {
    type Item = InitElement<'a>;

    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_init_el(input)
    }
}
