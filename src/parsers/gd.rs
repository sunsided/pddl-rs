//! Provides parsers for goal definitions.

use crate::parsers::{
    atomic_formula, literal, parse_term, parse_variable, prefix_expr, space_separated_list0,
    typed_list,
};
use crate::types::GD;
use nom::branch::alt;
use nom::character::complete::{char, multispace1};
use nom::combinator::map;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::parse_gd;
/// # use pddl::types::{AtomicFormula, EqualityAtomicFormula, GD, Literal, Term, TypedList, Variable};
///
/// // Atomic formula
/// assert_eq!(parse_gd("(= x y)"), Ok(("",
///     GD::AtomicFormula(
///         AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("y".into())
///         )
///     )
/// )));
///
/// // Literal
/// assert_eq!(parse_gd("(not (= x y))"), Ok(("",
///     GD::Literal(
///         Literal::new_not(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// )));
///
/// // Conjunction (and)
/// assert_eq!(parse_gd("(and (not (= x y)) (= x z))"), Ok(("",
///     GD::new_and([
///         GD::Literal(Literal::new_not(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )),
///         GD::AtomicFormula(AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("z".into())
///         ))
///     ])
/// )));
///
/// // Disjunction (or)
/// assert_eq!(parse_gd("(or (not (= x y)) (= x z))"), Ok(("",
///     GD::new_or([
///         GD::Literal(Literal::new_not(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )),
///         GD::AtomicFormula(AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("z".into())
///         ))
///     ])
/// )));
///
/// // Implication
/// assert_eq!(parse_gd("(imply (not (= x y)) (= x z))"), Ok(("",
///     GD::new_imply(
///         GD::Literal(Literal::new_not(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )),
///         GD::AtomicFormula(AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("z".into())
///         ))
///     )
/// )));
///
/// // Existential preconditions
/// assert_eq!(parse_gd("(exists (?x ?y) (not (= ?x ?y)))"), Ok(("",
///     GD::new_exists(
///         TypedList::from_iter([Variable::from_str("x").into(), Variable::from_str("y").into()]),
///         GD::Literal(Literal::new_not(
///             AtomicFormula::new_equality(
///                 Term::Variable("x".into()),
///                 Term::Variable("y".into())
///             )
///         ))
///     )
/// )));
///
/// // Universal preconditions
/// assert_eq!(parse_gd("(forall (?x ?y) (not (= ?x ?y)))"), Ok(("",
///     GD::new_forall(
///         TypedList::from_iter([Variable::from_str("x").into(), Variable::from_str("y").into()]),
///         GD::Literal(Literal::new_not(
///             AtomicFormula::new_equality(
///                 Term::Variable("x".into()),
///                 Term::Variable("y".into())
///             )
///         ))
///     )
/// )));
/// ```
pub fn parse_gd(input: &str) -> IResult<&str, GD> {
    let af = map(atomic_formula(parse_term), GD::new_atomic_formula);

    // :negative-preconditions
    let literal = map(literal(parse_term), GD::new_literal);

    let and = map(
        prefix_expr("and", space_separated_list0(parse_gd)),
        GD::new_and,
    );

    // :disjunctive-preconditions
    let or = map(
        prefix_expr("or", space_separated_list0(parse_gd)),
        GD::new_or,
    );

    // :disjunctive-preconditions
    let not = map(prefix_expr("not", parse_gd), GD::new_not);

    // :disjunctive-preconditions
    let imply = map(
        prefix_expr("imply", tuple((parse_gd, preceded(multispace1, parse_gd)))),
        GD::new_imply_tuple,
    );

    // :existential-preconditions
    let exists = map(
        prefix_expr(
            "exists",
            tuple((
                delimited(char('('), typed_list(parse_variable), char(')')),
                preceded(multispace1, parse_gd),
            )),
        ),
        GD::new_exists_tuple,
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                delimited(char('('), typed_list(parse_variable), char(')')),
                preceded(multispace1, parse_gd),
            )),
        ),
        GD::new_forall_tuple,
    );

    alt((af, literal, and, or, not, imply, exists, forall))(input)
}
