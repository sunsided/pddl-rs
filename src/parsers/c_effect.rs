//! Provides parsers for c-effects.

use crate::parsers::{parens, prefix_expr, typed_list, ParseResult, Span};
use crate::parsers::{parse_cond_effect, parse_effect, parse_gd, parse_p_effect, parse_variable};
use crate::types::CEffect;
use crate::{ForallCEffect, WhenCEffect};
use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::{preceded, tuple};

/// Parser that parses c-effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_c_effect, Span, UnwrapValue};
/// # use pddl::{AtomicFormula, CEffect, ConditionalEffect, Effects, EqualityAtomicFormula, GoalDefinition, PEffect, Predicate, Term, Variable};
/// # use pddl::{Typed, TypedList};
/// assert!(parse_c_effect(Span::new("(= x y)")).is_value(
///     CEffect::Effect(
///         PEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         )
///     )
/// ));
/// assert!(parse_c_effect(Span::new("(not (= ?a B))")).is_value(
///     CEffect::Effect(
///         PEffect::NotAtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Variable("a".into()),
///                 Term::Name("B".into()))
///             )
///         )
///     )
/// ));
///
/// assert!(parse_c_effect(Span::new("(forall (?a ?b) (= ?a ?b))")).is_value(
///     CEffect::new_forall(
///         TypedList::from_iter([
///             Typed::new_object(Variable::from_str("a")),
///             Typed::new_object(Variable::from_str("b")),
///         ]),
///         Effects::new(CEffect::Effect(
///             PEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Variable("a".into()),
///                     Term::Variable("b".into()))
///                 )
///             )
///         ))
///     )
/// ));
///
/// let input = r#"(when
///     (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
///     (and (person-is-happy ?p)))"#;
/// assert!(parse_c_effect(Span::new(input)).is_value(
///     CEffect::new_when(
///         GoalDefinition::new_and([
///             GoalDefinition::new_atomic_formula(
///                 AtomicFormula::new_predicate(
///                     Predicate::new("has-hot-chocolate".into()),
///                     [
///                         Term::Variable("p".into()),
///                         Term::Variable("c".into())
///                     ]
///                 )
///             ),
///             GoalDefinition::new_atomic_formula(
///                 AtomicFormula::new_predicate(
///                     Predicate::new("has-marshmallows".into()),
///                     [
///                         Term::Variable("c".into())
///                     ]
///                 )
///             ),
///         ]),
///         ConditionalEffect::from_iter([
///             PEffect::new(
///                 AtomicFormula::new_predicate(
///                     Predicate::new("person-is-happy".into()),
///                     [
///                         Term::Variable("p".into())
///                     ]
///                 )
///             )
///         ])
///     )
/// ));
/// ```
pub fn parse_c_effect<'a>(input: Span<'a>) -> ParseResult<'a, CEffect> {
    let p_effect = map(parse_p_effect, CEffect::from);
    let forall = map(parse_forall_c_effect, CEffect::from);
    let when = map(parse_when_c_effect, CEffect::from);

    alt((forall, when, p_effect))(input)
}

/// Parser that parses [`ForallCEffect`] values.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_forall_c_effect, preamble::*};
/// # use pddl::{AtomicFormula, CEffect, Effects, EqualityAtomicFormula, ForallCEffect, PEffect, Term, Variable};
/// # use pddl::{Typed, TypedList};
/// assert!(parse_forall_c_effect(Span::new("(forall (?a ?b) (= ?a ?b))")).is_value(
///     ForallCEffect::new(
///         TypedList::from_iter([
///             Typed::new_object(Variable::from_str("a")),
///             Typed::new_object(Variable::from_str("b")),
///         ]),
///         Effects::new(CEffect::Effect(
///             PEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Variable("a".into()),
///                     Term::Variable("b".into()))
///                 )
///             )
///         ))
///     )
/// ));
/// ```
pub fn parse_forall_c_effect<'a>(input: Span<'a>) -> ParseResult<'a, ForallCEffect> {
    map(
        prefix_expr(
            "forall",
            tuple((
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_effect),
            )),
        ),
        ForallCEffect::from,
    )(input)
}

/// Parser that parses c-effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_when_c_effect, preamble::*};
/// # use pddl::{AtomicFormula, CEffect, ConditionalEffect, Effects, EqualityAtomicFormula, GoalDefinition, PEffect, Predicate, Term, Variable, WhenCEffect};
/// # use pddl::{Typed, TypedList};
/// let input = r#"(when
///     (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
///     (and (person-is-happy ?p)))"#;
///
/// assert!(parse_when_c_effect(Span::new(input)).is_value(
///     WhenCEffect::new(
///         GoalDefinition::new_and([
///             GoalDefinition::new_atomic_formula(
///                 AtomicFormula::new_predicate(
///                     Predicate::new("has-hot-chocolate".into()),
///                     [
///                         Term::Variable("p".into()),
///                         Term::Variable("c".into())
///                     ]
///                 )
///             ),
///             GoalDefinition::new_atomic_formula(
///                 AtomicFormula::new_predicate(
///                     Predicate::new("has-marshmallows".into()),
///                     [
///                         Term::Variable("c".into())
///                     ]
///                 )
///             ),
///         ]),
///         ConditionalEffect::from_iter([
///             PEffect::new(
///                 AtomicFormula::new_predicate(
///                     Predicate::new("person-is-happy".into()),
///                     [
///                         Term::Variable("p".into())
///                     ]
///                 )
///             )
///         ])
///     )
/// ));
/// ```
pub fn parse_when_c_effect<'a>(input: Span<'a>) -> ParseResult<'a, WhenCEffect> {
    map(
        prefix_expr(
            "when",
            tuple((parse_gd, preceded(multispace1, parse_cond_effect))),
        ),
        WhenCEffect::from,
    )(input)
}

impl<'a> crate::parsers::Parser<'a> for CEffect<'a> {
    type Item = CEffect<'a>;

    /// See [`parse_c_effect`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_c_effect(input)
    }
}

impl<'a> crate::parsers::Parser<'a> for ForallCEffect<'a> {
    type Item = ForallCEffect<'a>;

    /// See [`parse_forall_c_effect`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_forall_c_effect(input)
    }
}

impl<'a> crate::parsers::Parser<'a> for WhenCEffect<'a> {
    type Item = WhenCEffect<'a>;

    /// See [`parse_when_c_effect`].
    fn parse(input: Span<'a>) -> ParseResult<Self::Item> {
        parse_when_c_effect(input)
    }
}
