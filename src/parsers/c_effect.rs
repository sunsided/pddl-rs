//! Provides parsers for conditional effects.

use nom::branch::alt;
use nom::character::complete::multispace1;
use nom::combinator::map;
use nom::sequence::preceded;
use nom::Parser;

use crate::parsers::{parens, prefix_expr, typed_list, ParseResult, Span};
use crate::parsers::{parse_effect_condition, parse_effect, parse_gd, parse_p_effect, parse_variable};
use crate::types::ConditionalEffect;
use crate::{ForallConditionalEffect, WhenConditionalEffect};

/// Parses conditional effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_c_effect, Span, UnwrapValue};
/// # use pddl::{AtomicFormula, ConditionalEffect, EffectCondition, Effects, EqualityAtomicFormula, GoalDefinition, PrimitiveEffect, Predicate, Term, Variable};
/// # use pddl::{Typed, TypedList};
/// assert!(parse_c_effect(Span::new("(= x y)")).is_value(
///     ConditionalEffect::Effect(
///         PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Name("x".into()),
///                 Term::Name("y".into()))
///             )
///         )
///     )
/// ));
/// assert!(parse_c_effect(Span::new("(not (= ?a B))")).is_value(
///     ConditionalEffect::Effect(
///         PrimitiveEffect::NotAtomicFormula(AtomicFormula::Equality(
///             EqualityAtomicFormula::new(
///                 Term::Variable("a".into()),
///                 Term::Name("B".into()))
///             )
///         )
///     )
/// ));
///
/// assert!(parse_c_effect(Span::new("(forall (?a ?b) (= ?a ?b))")).is_value(
///     ConditionalEffect::new_forall(
///         TypedList::from_iter([
///             Typed::new_object(Variable::new_string("a")),
///             Typed::new_object(Variable::new_string("b")),
///         ]),
///         Effects::new(ConditionalEffect::Effect(
///             PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
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
///     ConditionalEffect::new_when(
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
///         EffectCondition::from_iter([
///             PrimitiveEffect::new(
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
#[allow(deprecated)]
pub fn parse_c_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, ConditionalEffect> {
    let p_effect = map(parse_p_effect, ConditionalEffect::from);
    let forall = map(parse_forall_c_effect, ConditionalEffect::from);
    let when = map(parse_when_c_effect, ConditionalEffect::from);

    alt((forall, when, p_effect)).parse(input.into())
}

/// Parses [`ForallConditionalEffect`] values.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_forall_c_effect, preamble::*};
/// # use pddl::{AtomicFormula, ConditionalEffect, Effects, EqualityAtomicFormula, ForallConditionalEffect, PrimitiveEffect, Term, Variable};
/// # use pddl::{Typed, TypedList};
/// assert!(parse_forall_c_effect(Span::new("(forall (?a ?b) (= ?a ?b))")).is_value(
///     ForallConditionalEffect::new(
///         TypedList::from_iter([
///             Typed::new_object(Variable::new_string("a")),
///             Typed::new_object(Variable::new_string("b")),
///         ]),
///         Effects::new(ConditionalEffect::Effect(
///             PrimitiveEffect::AtomicFormula(AtomicFormula::Equality(
///                 EqualityAtomicFormula::new(
///                     Term::Variable("a".into()),
///                     Term::Variable("b".into()))
///                 )
///             )
///         ))
///     )
/// ));
/// ```
pub fn parse_forall_c_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, ForallConditionalEffect> {
    map(
        prefix_expr(
            "forall",
            (
                parens(typed_list(parse_variable)),
                preceded(multispace1, parse_effect),
            ),
        ),
        ForallConditionalEffect::from,
    )
    .parse(input.into())
}

/// Parses conditional effects.
///
/// ## Example
/// ```
/// # use pddl::parsers::{parse_when_c_effect, preamble::*};
/// # use pddl::{AtomicFormula, ConditionalEffect, EffectCondition, Effects, EqualityAtomicFormula, GoalDefinition, PrimitiveEffect, Predicate, Term, Variable, WhenConditionalEffect};
/// # use pddl::{Typed, TypedList};
/// let input = r#"(when
///     (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
///     (and (person-is-happy ?p)))"#;
///
/// assert!(parse_when_c_effect(Span::new(input)).is_value(
///     WhenConditionalEffect::new(
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
///         EffectCondition::from_iter([
///             PrimitiveEffect::new(
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
#[allow(deprecated)]
pub fn parse_when_c_effect<'a, T: Into<Span<'a>>>(input: T) -> ParseResult<'a, WhenConditionalEffect> {
    map(
        prefix_expr("when", (parse_gd, preceded(multispace1, parse_effect_condition))),
        WhenConditionalEffect::from,
    )
    .parse(input.into())
}

#[allow(deprecated)]
impl crate::parsers::Parser for ConditionalEffect {
    type Item = ConditionalEffect;

    /// Parses conditional effects.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{ConditionalEffect, EffectCondition, Effects, GoalDefinition, Parser, PrimitiveEffect, Typed, TypedList, Variable};
    /// let (_, value) = ConditionalEffect::parse("(= x y)").unwrap();
    /// assert_eq!(value,
    ///     ConditionalEffect::new_primitive_effect(
    ///         PrimitiveEffect::from_str("(= x y)").unwrap()
    ///     )
    /// );
    ///
    /// let (_, value) = ConditionalEffect::parse("(not (= ?a B))").unwrap();
    /// assert_eq!(value,
    ///     ConditionalEffect::new_primitive_effect(
    ///         PrimitiveEffect::from_str("(not (= ?a B))").unwrap()
    ///     )
    /// );
    ///
    /// let (_, value) = ConditionalEffect::parse("(forall (?a ?b) (= ?a ?b))").unwrap();
    /// assert_eq!(value,
    ///     ConditionalEffect::new_forall(
    ///         TypedList::from_iter([
    ///             Typed::new_object(Variable::new_string("a")),
    ///             Typed::new_object(Variable::new_string("b")),
    ///         ]),
    ///         Effects::from_str("(= ?a ?b)").unwrap()
    ///     )
    /// );
    ///
    /// let input = r#"(when
    ///     (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
    ///     (and (person-is-happy ?p)))"#;
    /// let (_, value) = ConditionalEffect::parse(input).unwrap();
    /// assert_eq!(value,
    ///     ConditionalEffect::new_when(
    ///         GoalDefinition::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))").unwrap(),
    ///         EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
    ///     )
    /// );
    /// ```
    ///
    /// ## See also
    /// See [`parse_c_effect`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_c_effect(input.into())
    }
}

impl crate::parsers::Parser for ForallConditionalEffect {
    type Item = ForallConditionalEffect;

    /// Parses [`ForallConditionalEffect`] values.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{Effects, ForallConditionalEffect, Parser, Typed, TypedList, Variable};
    /// let (_, value) = ForallConditionalEffect::parse("(forall (?a ?b) (= ?a ?b))").unwrap();
    /// assert_eq!(value,
    ///     ForallConditionalEffect::new(
    ///         TypedList::from_iter([
    ///             Typed::new_object(Variable::new_string("a")),
    ///             Typed::new_object(Variable::new_string("b")),
    ///         ]),
    ///         Effects::from_str("(= ?a ?b)").unwrap()
    ///     )
    /// );
    /// ```
    ///
    /// ## See also
    /// See [`parse_forall_c_effect`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_forall_c_effect(input)
    }
}

#[allow(deprecated)]
impl crate::parsers::Parser for WhenConditionalEffect {
    type Item = WhenConditionalEffect;

    /// Parses conditional effects.
    ///
    /// ## Example
    /// ```
    /// # use pddl::{EffectCondition, GoalDefinition, Parser, WhenConditionalEffect};
    /// let input = r#"(when
    ///     (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
    ///     (and (person-is-happy ?p)))"#;
    ///
    /// let (_, value) = WhenConditionalEffect::parse(input).unwrap();
    /// assert_eq!(value,
    ///     WhenConditionalEffect::new(
    ///         GoalDefinition::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))").unwrap(),
    ///         EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
    ///     )
    /// );
    /// ```
    ///
    /// ## See also
    /// See [`parse_when_c_effect`].
    fn parse<'a, S: Into<Span<'a>>>(input: S) -> ParseResult<'a, Self::Item> {
        parse_when_c_effect(input)
    }
}

#[cfg(test)]
#[allow(deprecated)]
mod tests {
    use crate::{
        ConditionalEffect, EffectCondition, Effects, ForallConditionalEffect, GoalDefinition, PrimitiveEffect, Parser, Typed,
        TypedList, Variable, WhenConditionalEffect,
    };

    #[test]
    fn test_parse() {
        let (_, value) = ConditionalEffect::parse("(= x y)").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_primitive_effect(PrimitiveEffect::from_str("(= x y)").unwrap())
        );

        let (_, value) = ConditionalEffect::parse("(not (= ?a B))").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_primitive_effect(PrimitiveEffect::from_str("(not (= ?a B))").unwrap())
        );

        let (_, value) = ConditionalEffect::parse("(forall (?a ?b) (= ?a ?b))").unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_forall(
                TypedList::from_iter([
                    Typed::new_object(Variable::new_string("a")),
                    Typed::new_object(Variable::new_string("b")),
                ]),
                Effects::from_str("(= ?a ?b)").unwrap()
            )
        );

        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;
        let (_, value) = ConditionalEffect::parse(input).unwrap();
        assert_eq!(
            value,
            ConditionalEffect::new_when(
                GoalDefinition::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))")
                    .unwrap(),
                EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
            )
        );
    }

    #[test]
    fn test_parse_forall() {
        let (_, value) = ForallConditionalEffect::parse("(forall (?a ?b) (= ?a ?b))").unwrap();
        assert_eq!(
            value,
            ForallConditionalEffect::new(
                TypedList::from_iter([
                    Typed::new_object(Variable::new_string("a")),
                    Typed::new_object(Variable::new_string("b")),
                ]),
                Effects::from_str("(= ?a ?b)").unwrap()
            )
        );
    }

    #[test]
    fn test_parse_when() {
        let input = r#"(when
            (and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))
            (and (person-is-happy ?p)))"#;

        let (_, value) = WhenConditionalEffect::parse(input).unwrap();
        assert_eq!(
            value,
            WhenConditionalEffect::new(
                GoalDefinition::from_str("(and (has-hot-chocolate ?p ?c) (has-marshmallows ?c))")
                    .unwrap(),
                EffectCondition::from_str("(and (person-is-happy ?p))").unwrap()
            )
        );
    }
}
