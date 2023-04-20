//! Provides parsers for p-effects.

use crate::parsers::{
    atomic_formula, parens, parse_assign_op, parse_f_exp, parse_f_head, parse_function_term,
    parse_term, prefix_expr,
};
use crate::types::PEffect;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

/// Parser combinator that parses p-effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::parse_p_effect;
/// # use pddl::types::{AssignOp, AtomicFormula, EqualityAtomicFormula, FExp, FHead, FunctionSymbol, FunctionTerm, PEffect, Term};
/// assert_eq!(parse_p_effect("(= x y)"), Ok(("",
///     PEffect::AtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Name("x".into()),
///             Term::Name("y".into()))
///         )
///     )
/// )));
///
/// assert_eq!(parse_p_effect("(not (= ?a B))"), Ok(("",
///     PEffect::NotAtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Variable("a".into()),
///             Term::Name("B".into()))
///         )
///     )
/// )));
///
/// assert_eq!(parse_p_effect("(assign fun-sym 1.23)"), Ok(("",
///     PEffect::new_numeric_fluent(
///         AssignOp::Assign,
///         FHead::new(FunctionSymbol::from_str("fun-sym")),
///         FExp::new_number(1.23)
///     )
/// )));
///
/// assert_eq!(parse_p_effect("(assign fun-sym 1.23)"), Ok(("",
///     PEffect::new_numeric_fluent(
///         AssignOp::Assign,
///         FHead::new(FunctionSymbol::from_str("fun-sym")),
///         FExp::new_number(1.23)
///     )
/// )));
///
/// assert_eq!(parse_p_effect("(assign (fun-sym) undefined)"), Ok(("",
///     PEffect::new_object_fluent(
///         FunctionTerm::new(FunctionSymbol::from_str("fun-sym"), []),
///         None
///     )
/// )));
///
/// assert_eq!(parse_p_effect("(assign (fun-sym) something)"), Ok(("",
///     PEffect::new_object_fluent(
///         FunctionTerm::new(FunctionSymbol::from_str("fun-sym"), []),
///         Some(Term::Name("something".into()))
///     )
/// )));
/// ```
pub fn parse_p_effect(input: &str) -> IResult<&str, PEffect> {
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

    alt((numeric, object_undefined, object, is_not, is))(input)
}
