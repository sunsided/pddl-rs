//! Provides parsers for goal definitions.

use crate::parsers::{
    atomic_formula, literal, parse_f_comp, parse_term, parse_variable, ParseResult, Span,
};
use crate::parsers::{parens, prefix_expr, space_separated_list0, typed_list};
use crate::types::GoalDefinition;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::{parse_gd, preamble::*};
/// # use pddl::{AtomicFormula, BinaryComp, BinaryOp, EqualityAtomicFormula, FComp, FExp, GoalDefinition, Literal, Term, Variable};
/// # use pddl::TypedList;
/// // Atomic formula
/// assert!(parse_gd("(= x y)".into()).is_value(
///     GoalDefinition::AtomicFormula(
///         AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("y".into())
///         )
///     )
/// ));
///
/// // Literal
/// assert!(parse_gd("(not (= x y))".into()).is_value(
///     GoalDefinition::new_not(
///         GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )
///     )
/// ));
///
/// // Conjunction (and)
/// assert!(parse_gd("(and (not (= x y)) (= x z))".into()).is_value(
///     GoalDefinition::new_and([
///         GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )),
///         GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("z".into())
///         ))
///     ])
/// ));
///
/// // Disjunction (or)
/// assert!(parse_gd("(or (not (= x y)) (= x z))".into()).is_value(
///     GoalDefinition::new_or([
///         GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )),
///         GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("z".into())
///         ))
///     ])
/// ));
///
/// // Implication
/// assert!(parse_gd("(imply (not (= x y)) (= x z))".into()).is_value(
///     GoalDefinition::new_imply(
///         GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Name("x".into()),
///                 Term::Name("y".into())
///             )
///         )),
///         GoalDefinition::AtomicFormula(AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("z".into())
///         ))
///     )
/// ));
///
/// // Existential preconditions
/// assert!(parse_gd("(exists (?x ?y) (not (= ?x ?y)))".into()).is_value(
///     GoalDefinition::new_exists(
///         TypedList::from_iter([Variable::from_str("x").into(), Variable::from_str("y").into()]),
///         GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Variable("x".into()),
///                 Term::Variable("y".into())
///             )
///         ))
///     )
/// ));
///
/// // Universal preconditions
/// assert!(parse_gd("(forall (?x ?y) (not (= ?x ?y)))".into()).is_value(
///     GoalDefinition::new_forall(
///         TypedList::from_iter([Variable::from_str("x").into(), Variable::from_str("y").into()]),
///         GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Variable("x".into()),
///                 Term::Variable("y".into())
///             )
///         ))
///     )
/// ));
///
/// assert!(parse_gd("(= (+ 1.23 2.34) (+ 1.23 2.34))".into()).is_value(
///     GoalDefinition::new_f_comp(
///         FComp::new(
///             BinaryComp::Equal,
///             FExp::new_binary_op(
///                 BinaryOp::Addition,
///                 FExp::new_number(1.23),
///                 FExp::new_number(2.34),
///             ),
///             FExp::new_binary_op(
///                 BinaryOp::Addition,
///                 FExp::new_number(1.23),
///                 FExp::new_number(2.34),
///             )
///         )
///     )
/// ));
/// ```
pub fn parse_gd(input: Span) -> ParseResult<GoalDefinition> {
    let af = map(
        atomic_formula(parse_term),
        GoalDefinition::new_atomic_formula,
    );

    // :negative-preconditions
    let literal = map(literal(parse_term), GoalDefinition::new_literal);

    let and = map(
        prefix_expr("and", space_separated_list0(parse_gd)),
        GoalDefinition::new_and,
    );

    // :disjunctive-preconditions
    let or = map(
        prefix_expr("or", space_separated_list0(parse_gd)),
        GoalDefinition::new_or,
    );

    // :disjunctive-preconditions
    let not = map(prefix_expr("not", parse_gd), GoalDefinition::new_not);

    // :disjunctive-preconditions
    let imply = map(
        prefix_expr("imply", tuple((parse_gd, preceded(multispace1, parse_gd)))),
        GoalDefinition::new_imply_tuple,
    );

    // :existential-preconditions
    let exists = map(
        prefix_expr(
            "exists",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_gd),
            )),
        ),
        GoalDefinition::new_exists_tuple,
    );

    // :universal-preconditions
    let forall = map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_gd),
            )),
        ),
        GoalDefinition::new_forall_tuple,
    );

    // :numeric-fluents
    let f_comp = map(parse_f_comp, GoalDefinition::new_f_comp);

    alt((and, or, not, imply, exists, forall, af, literal, f_comp))(input)
}

impl<'a> crate::parsers::Parser<'a> for GoalDefinition<'a> {
    type Item = GoalDefinition<'a>;

    /// See [`parse_gd`].
    fn parse<S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_gd(input.into())
    }
}
