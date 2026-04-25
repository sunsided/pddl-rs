//! Contains goal definitions via the [`GoalDefinition`] type.

use std::ops::Not;

use crate::types::TermLiteral;
use crate::types::{AtomicFormula, FluentComparison, Term, TypedVariables};

/// A goal definition.
///
/// ## Usage
/// Used by [`GoalDefinition`] itself, as well as [`PreferenceGoalDefinition`](crate::PreferenceGoalDefinition), [`ConditionalEffect`](crate::ConditionalEffect),
/// [`TimedGoalDefinition`](crate::TimedGoalDefinition), [`DerivedPredicate`](crate::DerivedPredicate) and
/// [`ConstraintGoalDefinitionInner`](crate::ConstraintGoalDefinitionInner).
#[derive(Debug, Clone, PartialEq)]
pub enum GoalDefinition {
    AtomicFormula(AtomicFormula<Term>),
    /// ## Requirements
    /// Requires [Negative Preconditions](crate::Requirement::NegativePreconditions).
    Literal(TermLiteral),
    And(Vec<GoalDefinition>),
    /// ## Requirements
    /// Requires [Disjunctive Preconditions](crate::Requirement::DisjunctivePreconditions).
    Or(Vec<GoalDefinition>),
    /// ## Requirements
    /// Requires [Disjunctive Preconditions](crate::Requirement::DisjunctivePreconditions).
    Not(Box<GoalDefinition>),
    /// ## Requirements
    /// Requires [Disjunctive Preconditions](crate::Requirement::DisjunctivePreconditions).
    Imply(Box<GoalDefinition>, Box<GoalDefinition>),
    /// ## Requirements
    /// Requires [Existential Preconditions](crate::Requirement::ExistentialPreconditions).
    Exists(TypedVariables, Box<GoalDefinition>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    ForAll(TypedVariables, Box<GoalDefinition>),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    FluentComparison(FluentComparison),
}

impl GoalDefinition {
    #[inline(always)]
    #[doc(alias = "new_atomic_formula")]
    pub const fn atomic_formula(value: AtomicFormula<Term>) -> Self {
        Self::AtomicFormula(value)
    }

    pub fn new_atomic_formula(value: AtomicFormula<Term>) -> Self {
        Self::atomic_formula(value)
    }

    #[inline(always)]
    #[doc(alias = "new_literal")]
    pub const fn literal(value: TermLiteral) -> Self {
        Self::Literal(value)
    }

    pub fn new_literal(value: TermLiteral) -> Self {
        Self::literal(value)
    }

    #[inline(always)]
    #[doc(alias = "new_and")]
    pub fn and<T: IntoIterator<Item = GoalDefinition>>(values: T) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(values.into_iter().collect())
    }

    pub fn new_and<T: IntoIterator<Item = GoalDefinition>>(values: T) -> Self {
        Self::and(values)
    }

    #[inline(always)]
    #[doc(alias = "new_or")]
    pub fn or<T: IntoIterator<Item = GoalDefinition>>(values: T) -> Self {
        // TODO: Flatten `(or (or a b) (or x y))` into `(or a b c y)`.
        Self::Or(values.into_iter().collect())
    }

    pub fn new_or<T: IntoIterator<Item = GoalDefinition>>(values: T) -> Self {
        Self::or(values)
    }

    #[allow(clippy::should_implement_trait)]
    #[inline(always)]
    #[doc(alias = "new_not")]
    pub fn not(value: GoalDefinition) -> Self {
        Self::Not(Box::new(value))
    }
}

impl Not for GoalDefinition {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

impl GoalDefinition {
    pub fn new_not(value: GoalDefinition) -> Self {
        Self::not(value)
    }

    #[inline(always)]
    #[doc(alias = "new_imply_tuple")]
    pub fn imply_tuple(tuple: (GoalDefinition, GoalDefinition)) -> Self {
        Self::imply(tuple.0, tuple.1)
    }

    pub fn new_imply_tuple(tuple: (GoalDefinition, GoalDefinition)) -> Self {
        Self::imply_tuple(tuple)
    }

    #[inline(always)]
    #[doc(alias = "new_imply")]
    pub fn imply(a: GoalDefinition, b: GoalDefinition) -> Self {
        Self::Imply(Box::new(a), Box::new(b))
    }

    pub fn new_imply(a: GoalDefinition, b: GoalDefinition) -> Self {
        Self::imply(a, b)
    }

    #[inline(always)]
    #[doc(alias = "new_exists_tuple")]
    pub fn exists_tuple(tuple: (TypedVariables, GoalDefinition)) -> Self {
        Self::exists(tuple.0, tuple.1)
    }

    pub fn new_exists_tuple(tuple: (TypedVariables, GoalDefinition)) -> Self {
        Self::exists_tuple(tuple)
    }

    #[inline(always)]
    #[doc(alias = "new_exists")]
    pub fn exists(variables: TypedVariables, gd: GoalDefinition) -> Self {
        Self::Exists(variables, Box::new(gd))
    }

    pub fn new_exists(variables: TypedVariables, gd: GoalDefinition) -> Self {
        Self::exists(variables, gd)
    }

    #[inline(always)]
    #[doc(alias = "new_forall_tuple")]
    pub fn forall_tuple(tuple: (TypedVariables, GoalDefinition)) -> Self {
        Self::forall(tuple.0, tuple.1)
    }

    pub fn new_forall_tuple(tuple: (TypedVariables, GoalDefinition)) -> Self {
        Self::forall_tuple(tuple)
    }

    #[inline(always)]
    #[doc(alias = "new_forall")]
    pub fn r#forall(variables: TypedVariables, gd: GoalDefinition) -> Self {
        Self::ForAll(variables, Box::new(gd))
    }

    pub fn new_forall(variables: TypedVariables, gd: GoalDefinition) -> Self {
        Self::r#forall(variables, gd)
    }

    #[inline(always)]
    #[doc(alias = "new_f_comp")]
    pub const fn fluent_comparison(f_comp: FluentComparison) -> Self {
        Self::FluentComparison(f_comp)
    }

    pub fn new_f_comp(f_comp: FluentComparison) -> Self {
        Self::fluent_comparison(f_comp)
    }

    pub fn is_empty(&self) -> bool {
        match self {
            GoalDefinition::AtomicFormula(_) => false,
            GoalDefinition::Literal(_) => false,
            GoalDefinition::And(x) => x.iter().all(|y| y.is_empty()),
            GoalDefinition::Or(x) => x.iter().all(|y| y.is_empty()),
            GoalDefinition::Not(x) => x.is_empty(),
            GoalDefinition::Imply(x, y) => x.is_empty() && y.is_empty(),
            GoalDefinition::Exists(_, x) => x.is_empty(),
            GoalDefinition::ForAll(_, x) => x.is_empty(),
            GoalDefinition::FluentComparison(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        AtomicFormula, BinaryComparison, FluentComparison, FluentExpression, Number, Predicate,
        Term,
    };

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let af = AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new());
        let _ = GoalDefinition::new_atomic_formula(af.clone());

        let literal = TermLiteral::new_not(AtomicFormula::predicate(
            Predicate::string("x"),
            Vec::<Term>::new(),
        ));
        let _ = GoalDefinition::new_literal(literal.clone());

        let _ = GoalDefinition::new_and(vec![GoalDefinition::AtomicFormula(af.clone())]);
        let _ = GoalDefinition::new_or(vec![GoalDefinition::AtomicFormula(af.clone())]);
        let _ = GoalDefinition::new_not(GoalDefinition::AtomicFormula(af.clone()));

        let _ = GoalDefinition::new_imply_tuple((
            GoalDefinition::AtomicFormula(af.clone()),
            GoalDefinition::AtomicFormula(af.clone()),
        ));
        let _ = GoalDefinition::new_imply(
            GoalDefinition::AtomicFormula(af.clone()),
            GoalDefinition::AtomicFormula(af.clone()),
        );

        let vars = TypedVariables::default();
        let _ = GoalDefinition::new_exists_tuple((
            vars.clone(),
            GoalDefinition::AtomicFormula(af.clone()),
        ));
        let _ = GoalDefinition::new_exists(vars.clone(), GoalDefinition::AtomicFormula(af.clone()));
        let _ = GoalDefinition::new_forall_tuple((
            vars.clone(),
            GoalDefinition::AtomicFormula(af.clone()),
        ));
        let _ = GoalDefinition::new_forall(vars, GoalDefinition::AtomicFormula(af.clone()));

        let f_comp = FluentComparison::new(
            BinaryComparison::GreaterThan,
            FluentExpression::number(Number::from(1)),
            FluentExpression::number(Number::from(0)),
        );
        let _ = GoalDefinition::new_f_comp(f_comp);
    }

    #[test]
    fn not_trait() {
        let gd = GoalDefinition::AtomicFormula(AtomicFormula::predicate(
            Predicate::string("clear"),
            Vec::<Term>::new(),
        ));
        let negated = !gd;
        assert!(matches!(negated, GoalDefinition::Not(_)));
    }

    #[test]
    fn is_empty_non_empty_variants() {
        let af = AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new());
        assert!(!GoalDefinition::AtomicFormula(af).is_empty());

        let literal = TermLiteral::new(AtomicFormula::predicate(
            Predicate::string("x"),
            Vec::<Term>::new(),
        ));
        assert!(!GoalDefinition::Literal(literal).is_empty());

        let gd = GoalDefinition::AtomicFormula(AtomicFormula::predicate(
            Predicate::string("on"),
            Vec::<Term>::new(),
        ));
        assert!(!GoalDefinition::Not(Box::new(gd.clone())).is_empty());
        assert!(!GoalDefinition::Imply(Box::new(gd.clone()), Box::new(gd.clone())).is_empty());
        assert!(
            !GoalDefinition::Exists(TypedVariables::default(), Box::new(gd.clone())).is_empty()
        );
        assert!(
            !GoalDefinition::ForAll(TypedVariables::default(), Box::new(gd.clone())).is_empty()
        );

        let f_comp = FluentComparison::new(
            BinaryComparison::GreaterThan,
            FluentExpression::number(Number::from(1)),
            FluentExpression::number(Number::from(0)),
        );
        assert!(!GoalDefinition::FluentComparison(f_comp).is_empty());
    }

    #[test]
    fn is_empty_recursive() {
        let empty_and = GoalDefinition::and(Vec::new());
        assert!(empty_and.is_empty());

        let empty_or = GoalDefinition::or(Vec::new());
        assert!(empty_or.is_empty());

        let nested_empty = GoalDefinition::Not(Box::new(GoalDefinition::and(Vec::new())));
        assert!(nested_empty.is_empty());

        let non_empty_and = GoalDefinition::and(vec![GoalDefinition::AtomicFormula(
            AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new()),
        )]);
        assert!(!non_empty_and.is_empty());
    }
}
