//! Provides parsers for p-effects.

use crate::parsers::{
    atomic_formula, parse_assign_op, parse_f_exp, parse_f_head, parse_function_term, parse_term,
    ParseResult, Span,
};
use crate::parsers::{parens, prefix_expr};
use crate::types::PEffect;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated, tuple};

/// Parses p-effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_p_effect, preamble::*};
/// # use pddl::{AssignOp, AtomicFormula, EqualityAtomicFormula, FExp, FHead, FunctionSymbol, FunctionTerm, PEffect, Term};
/// assert!(parse_p_effect("(= x y)").is_value(
///     PEffect::AtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Name("x".into()),
///             Term::Name("y".into()))
///         )
///     )
/// ));
///
/// assert!(parse_p_effect("(not (= ?a B))").is_value(
///     PEffect::NotAtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Variable("a".into()),
///             Term::Name("B".into()))
///         )
///     )
/// ));
///
/// assert!(parse_p_effect("(assign fun-sym 1.23)").is_value(
///     PEffect::new_numeric_fluent(
///         AssignOp::Assign,
///         FHead::new(FunctionSymbol::from_str("fun-sym")),
///         FExp::new_number(1.23)
///     )
/// ));
///
/// assert!(parse_p_effect("(assign fun-sym 1.23)").is_value(
///     PEffect::new_numeric_fluent(
///         AssignOp::Assign,
///         FHead::new(FunctionSymbol::from_str("fun-sym")),
///         FExp::new_number(1.23)
///     )
/// ));
///
/// assert!(parse_p_effect("(assign (fun-sym) undefined)").is_value(
///     PEffect::new_object_fluent(
///         FunctionTerm::new(FunctionSymbol::from_str("fun-sym"), []),
///         None
///     )
/// ));
///
/// assert!(parse_p_effect("(assign (fun-sym) something)").is_value(
///     PEffect::new_object_fluent(
///         FunctionTerm::new(FunctionSymbol::from_str("fun-sym"), []),
///         Some(Term::Name("something".into()))
///     )
/// ));
/// ```
pub fn parse_p_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, PEffect<'a>> {
    let is = map(atomic_formula(parse_term), |af| PEffect::new(af));
    let is_not = map(prefix_expr("not", atomic_formula(parse_term)), |af| {
        PEffect::new_not(af)
    });

    // :numeric-fluents
    let numeric = map(
        parens(tuple((
            parse_assign_op,
            preceded(multispace1, parse_f_head),
            preceded(multispace1, parse_f_exp),
        ))),
        |(op, head, exp)| PEffect::new_numeric_fluent(op, head, exp),
    );

    // :object-fluents
    let object_undefined = map(
        prefix_expr(
            "assign",
            terminated(parse_function_term, tuple((multispace1, tag("undefined")))),
        ),
        |f_term| PEffect::new_object_fluent(f_term, None),
    );
    let object = map(
        prefix_expr(
            "assign",
            tuple((parse_function_term, preceded(multispace1, parse_term))),
        ),
        |(f_term, term)| PEffect::new_object_fluent(f_term, Some(term)),
    );

    alt((object_undefined, object, numeric, is_not, is))(input.into())
}

impl<'a> crate::parsers::Parser<'a> for PEffect<'a> {
    type Item = PEffect<'a>;

    /// See [`parse_p_effect`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_p_effect(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "(can-move ?from-waypoint ?to-waypoint)";
        let (_, _effect) = parse_p_effect(Span::new(input)).unwrap();
    }
}
