//! Provides parsers for p-effects.

use crate::parsers::{
    atomic_formula, parse_assign_op, parse_f_exp, parse_f_head, parse_function_term, parse_term,
    ParseResult, Span,
};
use crate::parsers::{parens, prefix_expr};
use crate::types::PrimitiveEffect;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use nom::Parser;

/// Parses p-effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_p_effect, preamble::*};
/// # use pddl::{AssignOp, AtomicFormula, EqualityAtomicFormula, FluentExpression, FunctionHead, FunctionSymbol, FunctionTerm, PrimitiveEffect, Term};
/// assert!(parse_p_effect("(= x y)").is_value(
///     PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Name("x".into()),
///             Term::Name("y".into()))
///         )
///     )
/// ));
///
/// assert!(parse_p_effect("(not (= ?a B))").is_value(
///     PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(
///         EqualityAtomicFormula::new(
///             Term::Variable("a".into()),
///             Term::Name("B".into()))
///         )
///     )
/// ));
///
/// assert!(parse_p_effect("(assign fun-sym 1.23)").is_value(
///     PrimitiveEffect::numeric_fluent(
///         AssignOp::Assign,
///         FunctionHead::new(FunctionSymbol::string("fun-sym")),
///         FluentExpression::number(1.23)
///     )
/// ));
///
/// assert!(parse_p_effect("(assign fun-sym 1.23)").is_value(
///     PrimitiveEffect::numeric_fluent(
///         AssignOp::Assign,
///         FunctionHead::new(FunctionSymbol::string("fun-sym")),
///         FluentExpression::number(1.23)
///     )
/// ));
///
/// assert!(parse_p_effect("(assign (fun-sym) undefined)").is_value(
///     PrimitiveEffect::object_fluent(
///         FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
///         None
///     )
/// ));
///
/// assert!(parse_p_effect("(assign (fun-sym) something)").is_value(
///     PrimitiveEffect::object_fluent(
///         FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
///         Some(Term::Name("something".into()))
///     )
/// ));
/// ```
pub fn parse_p_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, PrimitiveEffect> {
    let is = map(atomic_formula(parse_term), PrimitiveEffect::new);
    let is_not = map(prefix_expr("not", atomic_formula(parse_term)), |af| {
        PrimitiveEffect::not(af)
    });

    // :numeric-fluents
    let numeric = map(
        parens((
            parse_assign_op,
            preceded(multispace1, parse_f_head),
            preceded(multispace1, parse_f_exp),
        )),
        |(op, head, exp)| PrimitiveEffect::numeric_fluent(op, head, exp),
    );

    // :object-fluents
    let object_undefined = map(
        prefix_expr(
            "assign",
            terminated(parse_function_term, (multispace1, tag("undefined"))),
        ),
        |f_term| PrimitiveEffect::object_fluent(f_term, None),
    );
    let object = map(
        prefix_expr(
            "assign",
            (parse_function_term, preceded(multispace1, parse_term)),
        ),
        |(f_term, term)| PrimitiveEffect::object_fluent(f_term, Some(term)),
    );

    alt((is_not, object_undefined, object, numeric, is)).parse(input.into())
}

impl crate::parsers::Parser for PrimitiveEffect {
    type Item = PrimitiveEffect;

    /// See [`parse_p_effect`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_p_effect(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::UnwrapValue;
    use crate::{
        AssignOp, AtomicFormula, EqualityAtomicFormula, FluentExpression, FunctionHead,
        FunctionSymbol, FunctionTerm, Parser, Term,
    };

    #[test]
    fn it_works() {
        let input = "(can-move ?from-waypoint ?to-waypoint)";
        let (_, _effect) = parse_p_effect(Span::new(input)).unwrap();
    }

    #[test]
    fn not_works() {
        let input = "(not (at B ?m))";
        let mut is_not = map(prefix_expr("not", atomic_formula(parse_term)), |af| {
            PrimitiveEffect::not(af)
        });

        let result: Result<(_, _), _> = nom::Parser::parse(&mut is_not, input.into());
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse() {
        assert!(
            PrimitiveEffect::parse("(= x y)").is_value(PrimitiveEffect::AtomicFormula(
                AtomicFormula::Equality(EqualityAtomicFormula::new(
                    Term::Name("x".into()),
                    Term::Name("y".into())
                ))
            ))
        );

        assert!(PrimitiveEffect::parse("(not (= ?a B))").is_value(
            PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(EqualityAtomicFormula::new(
                Term::Variable("a".into()),
                Term::Name("B".into())
            )))
        ));

        assert!(PrimitiveEffect::parse("(assign fun-sym 1.23)").is_value(
            PrimitiveEffect::numeric_fluent(
                AssignOp::Assign,
                FunctionHead::new(FunctionSymbol::string("fun-sym")),
                FluentExpression::number(1.23)
            )
        ));

        assert!(PrimitiveEffect::parse("(assign fun-sym 1.23)").is_value(
            PrimitiveEffect::numeric_fluent(
                AssignOp::Assign,
                FunctionHead::new(FunctionSymbol::string("fun-sym")),
                FluentExpression::number(1.23)
            )
        ));

        assert!(
            PrimitiveEffect::parse("(assign (fun-sym) undefined)").is_value(
                PrimitiveEffect::object_fluent(
                    FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
                    None
                )
            )
        );

        assert!(
            PrimitiveEffect::parse("(assign (fun-sym) something)").is_value(
                PrimitiveEffect::object_fluent(
                    FunctionTerm::new(FunctionSymbol::string("fun-sym"), []),
                    Some(Term::Name("something".into()))
                )
            )
        );
    }
}
