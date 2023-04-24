//! Provides parsers for goal definitions.

use crate::parsers::{atomic_formula, literal, parse_f_comp, parse_term, parse_variable};
use crate::parsers::{parens, prefix_expr, space_separated_list0, typed_list};
use crate::types::GoalDefinition;
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};
use nom::IResult;

/// Parser for goal definitions.
///
/// ## Examples
/// ```
/// # use pddl::parsers::parse_gd;
/// # use pddl::{AtomicFormula, BinaryComp, BinaryOp, EqualityAtomicFormula, FComp, FExp, GoalDefinition, Literal, Term, Variable};
/// # use pddl::TypedList;
/// // Atomic formula
/// assert_eq!(parse_gd("(= x y)"), Ok(("",
///     GoalDefinition::AtomicFormula(
///         AtomicFormula::new_equality(
///             Term::Name("x".into()),
///             Term::Name("y".into())
///         )
///     )
/// )));
///
/// // Literal
/// assert_eq!(parse_gd("(not (= x y))"), Ok(("",
///     GoalDefinition::new_not(
///         GoalDefinition::new_atomic_formula(
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
/// )));
///
/// // Disjunction (or)
/// assert_eq!(parse_gd("(or (not (= x y)) (= x z))"), Ok(("",
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
/// )));
///
/// // Implication
/// assert_eq!(parse_gd("(imply (not (= x y)) (= x z))"), Ok(("",
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
/// )));
///
/// // Existential preconditions
/// assert_eq!(parse_gd("(exists (?x ?y) (not (= ?x ?y)))"), Ok(("",
///     GoalDefinition::new_exists(
///         TypedList::from_iter([Variable::from_str("x").into(), Variable::from_str("y").into()]),
///         GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
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
///     GoalDefinition::new_forall(
///         TypedList::from_iter([Variable::from_str("x").into(), Variable::from_str("y").into()]),
///         GoalDefinition::new_not(GoalDefinition::new_atomic_formula(
///             AtomicFormula::new_equality(
///                 Term::Variable("x".into()),
///                 Term::Variable("y".into())
///             )
///         ))
///     )
/// )));
///
/// assert_eq!(parse_gd("(= (+ 1.23 2.34) (+ 1.23 2.34))"), Ok(("",
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
/// )));
/// ```
pub fn parse_gd(input: &str) -> IResult<&str, GoalDefinition> {
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
    fn parse(input: &'a str) -> IResult<&str, Self::Item> {
        parse_gd(input)
    }
}
