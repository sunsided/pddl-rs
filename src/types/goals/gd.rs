//! Contains goal descriptions via the [`GoalDescription`] type.

use std::ops::Not;

use crate::types::TermLiteral;
use crate::types::{AtomicFormula, FluentComparison, Term, TypedVariables};

/// A goal description.
///
/// ## Usage
/// Used by [`GoalDescription`] itself, as well as [`PreferenceGoalDescription`](crate::PreferenceGoalDescription), [`ConditionalEffect`](crate::ConditionalEffect),
/// [`TimedGoalDescription`](crate::TimedGoalDescription), [`DerivedPredicate`](crate::DerivedPredicate) and
/// [`ConstraintGoalDescriptionInner`](crate::ConstraintGoalDescriptionInner).
#[derive(Debug, Clone, PartialEq)]
pub enum GoalDescription {
    /// An atomic formula, `<atomic formula(term)>`.
    AtomicFormula(AtomicFormula<Term>),
    /// ## Requirements
    /// Requires [Negative Preconditions](crate::Requirement::NegativePreconditions).
    Literal(TermLiteral),
    /// A conjunction, `(and <GD>*)`.
    And(Vec<GoalDescription>),
    /// ## Requirements
    /// Requires [Disjunctive Preconditions](crate::Requirement::DisjunctivePreconditions).
    Or(Vec<GoalDescription>),
    /// ## Requirements
    /// Requires [Disjunctive Preconditions](crate::Requirement::DisjunctivePreconditions).
    Not(Box<GoalDescription>),
    /// ## Requirements
    /// Requires [Disjunctive Preconditions](crate::Requirement::DisjunctivePreconditions).
    Imply(Box<GoalDescription>, Box<GoalDescription>),
    /// ## Requirements
    /// Requires [Existential Preconditions](crate::Requirement::ExistentialPreconditions).
    Exists(TypedVariables, Box<GoalDescription>),
    /// ## Requirements
    /// Requires [Universal Preconditions](crate::Requirement::UniversalPreconditions).
    ForAll(TypedVariables, Box<GoalDescription>),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    FluentComparison(FluentComparison),
}

impl GoalDescription {
    /// Creates a goal from an atomic formula.
    #[inline]
    #[doc(alias = "new_atomic_formula")]
    #[must_use]
    pub const fn atomic_formula(value: AtomicFormula<Term>) -> Self {
        Self::AtomicFormula(value)
    }

    /// Alias for [`Self::atomic_formula`].
    #[must_use]
    pub fn new_atomic_formula(value: AtomicFormula<Term>) -> Self {
        Self::atomic_formula(value)
    }

    /// Creates a goal from a literal, e.g. `(not <atomic formula(term)>)`.
    #[inline]
    #[doc(alias = "new_literal")]
    #[must_use]
    pub const fn literal(value: TermLiteral) -> Self {
        Self::Literal(value)
    }

    /// Alias for [`Self::literal`].
    #[must_use]
    pub fn new_literal(value: TermLiteral) -> Self {
        Self::literal(value)
    }

    /// Creates a conjunction `(and <GD>*)`.
    #[inline]
    #[doc(alias = "new_and")]
    #[must_use]
    pub fn and<T: IntoIterator<Item = GoalDescription>>(values: T) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(values.into_iter().collect())
    }

    /// Alias for [`Self::and`].
    #[must_use]
    pub fn new_and<T: IntoIterator<Item = GoalDescription>>(values: T) -> Self {
        Self::and(values)
    }

    /// Creates a disjunction `(or <GD>*)`.
    #[inline]
    #[doc(alias = "new_or")]
    #[must_use]
    pub fn or<T: IntoIterator<Item = GoalDescription>>(values: T) -> Self {
        // TODO: Flatten `(or (or a b) (or x y))` into `(or a b c y)`.
        Self::Or(values.into_iter().collect())
    }

    /// Alias for [`Self::or`].
    #[must_use]
    pub fn new_or<T: IntoIterator<Item = GoalDescription>>(values: T) -> Self {
        Self::or(values)
    }

    /// Creates a negation `(not <GD>)`.
    #[allow(clippy::should_implement_trait)]
    #[inline]
    #[doc(alias = "new_not")]
    #[must_use]
    pub fn not(value: GoalDescription) -> Self {
        Self::Not(Box::new(value))
    }
}

impl Not for GoalDescription {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self::Not(Box::new(self))
    }
}

impl GoalDescription {
    /// Alias for [`Self::not`].
    #[must_use]
    pub fn new_not(value: GoalDescription) -> Self {
        Self::not(value)
    }

    /// Creates an implication `(imply <GD> <GD>)` from a `(premise, conclusion)` tuple.
    #[inline]
    #[doc(alias = "new_imply_tuple")]
    #[must_use]
    pub fn imply_tuple(tuple: (GoalDescription, GoalDescription)) -> Self {
        Self::imply(tuple.0, tuple.1)
    }

    /// Alias for [`Self::imply_tuple`].
    #[must_use]
    pub fn new_imply_tuple(tuple: (GoalDescription, GoalDescription)) -> Self {
        Self::imply_tuple(tuple)
    }

    /// Creates an implication `(imply a b)`.
    #[inline]
    #[doc(alias = "new_imply")]
    #[must_use]
    pub fn imply(a: GoalDescription, b: GoalDescription) -> Self {
        Self::Imply(Box::new(a), Box::new(b))
    }

    /// Alias for [`Self::imply`].
    #[must_use]
    pub fn new_imply(a: GoalDescription, b: GoalDescription) -> Self {
        Self::imply(a, b)
    }

    /// Creates an existential quantification `(exists (<variables>) <GD>)` from a tuple.
    #[inline]
    #[doc(alias = "new_exists_tuple")]
    #[must_use]
    pub fn exists_tuple(tuple: (TypedVariables, GoalDescription)) -> Self {
        Self::exists(tuple.0, tuple.1)
    }

    /// Alias for [`Self::exists_tuple`].
    #[must_use]
    pub fn new_exists_tuple(tuple: (TypedVariables, GoalDescription)) -> Self {
        Self::exists_tuple(tuple)
    }

    /// Creates an existential quantification `(exists (<variables>) <GD>)`.
    #[inline]
    #[doc(alias = "new_exists")]
    #[must_use]
    pub fn exists(variables: TypedVariables, gd: GoalDescription) -> Self {
        Self::Exists(variables, Box::new(gd))
    }

    /// Alias for [`Self::exists`].
    #[must_use]
    pub fn new_exists(variables: TypedVariables, gd: GoalDescription) -> Self {
        Self::exists(variables, gd)
    }

    /// Creates a universal quantification `(forall (<variables>) <GD>)` from a tuple.
    #[inline]
    #[doc(alias = "new_forall_tuple")]
    #[must_use]
    pub fn forall_tuple(tuple: (TypedVariables, GoalDescription)) -> Self {
        Self::forall(tuple.0, tuple.1)
    }

    /// Alias for [`Self::forall_tuple`].
    #[must_use]
    pub fn new_forall_tuple(tuple: (TypedVariables, GoalDescription)) -> Self {
        Self::forall_tuple(tuple)
    }

    /// Creates a universal quantification `(forall (<variables>) <GD>)`.
    #[inline]
    #[doc(alias = "new_forall")]
    #[must_use]
    pub fn r#forall(variables: TypedVariables, gd: GoalDescription) -> Self {
        Self::ForAll(variables, Box::new(gd))
    }

    /// Alias for [`Self::forall`].
    #[must_use]
    pub fn new_forall(variables: TypedVariables, gd: GoalDescription) -> Self {
        Self::r#forall(variables, gd)
    }

    /// Creates a numeric fluent comparison, e.g. `(> <f-exp> <f-exp>)`.
    #[inline]
    #[doc(alias = "new_f_comp")]
    #[must_use]
    pub const fn fluent_comparison(f_comp: FluentComparison) -> Self {
        Self::FluentComparison(f_comp)
    }

    /// Alias for [`Self::fluent_comparison`].
    #[must_use]
    pub fn new_f_comp(f_comp: FluentComparison) -> Self {
        Self::fluent_comparison(f_comp)
    }

    /// Returns `true` if this goal contains no atomic formula, literal or comparison,
    /// e.g. `(and)`.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            GoalDescription::AtomicFormula(_)
            | GoalDescription::Literal(_)
            | GoalDescription::FluentComparison(_) => false,
            GoalDescription::And(x) | GoalDescription::Or(x) => {
                x.iter().all(GoalDescription::is_empty)
            }
            GoalDescription::Not(x)
            | GoalDescription::Exists(_, x)
            | GoalDescription::ForAll(_, x) => x.is_empty(),
            GoalDescription::Imply(x, y) => x.is_empty() && y.is_empty(),
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
    use assert2::check;

    #[test]
    #[allow(deprecated)]
    fn deprecated_new_constructors() {
        let af = AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new());
        let _ = GoalDescription::new_atomic_formula(af.clone());

        let literal = TermLiteral::new_not(AtomicFormula::predicate(
            Predicate::string("x"),
            Vec::<Term>::new(),
        ));
        let _ = GoalDescription::new_literal(literal.clone());

        let _ = GoalDescription::new_and(vec![GoalDescription::AtomicFormula(af.clone())]);
        let _ = GoalDescription::new_or(vec![GoalDescription::AtomicFormula(af.clone())]);
        let _ = GoalDescription::new_not(GoalDescription::AtomicFormula(af.clone()));

        let _ = GoalDescription::new_imply_tuple((
            GoalDescription::AtomicFormula(af.clone()),
            GoalDescription::AtomicFormula(af.clone()),
        ));
        let _ = GoalDescription::new_imply(
            GoalDescription::AtomicFormula(af.clone()),
            GoalDescription::AtomicFormula(af.clone()),
        );

        let vars = TypedVariables::default();
        let _ = GoalDescription::new_exists_tuple((
            vars.clone(),
            GoalDescription::AtomicFormula(af.clone()),
        ));
        let _ =
            GoalDescription::new_exists(vars.clone(), GoalDescription::AtomicFormula(af.clone()));
        let _ = GoalDescription::new_forall_tuple((
            vars.clone(),
            GoalDescription::AtomicFormula(af.clone()),
        ));
        let _ = GoalDescription::new_forall(vars, GoalDescription::AtomicFormula(af.clone()));

        let f_comp = FluentComparison::new(
            BinaryComparison::GreaterThan,
            FluentExpression::number(Number::from(1)),
            FluentExpression::number(Number::from(0)),
        );
        let _ = GoalDescription::new_f_comp(f_comp);
    }

    #[test]
    fn not_trait() {
        let gd = GoalDescription::AtomicFormula(AtomicFormula::predicate(
            Predicate::string("clear"),
            Vec::<Term>::new(),
        ));
        let negated = !gd;
        check!(let GoalDescription::Not(_) = negated);
    }

    #[test]
    fn is_empty_non_empty_variants() {
        let af = AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new());
        check!(!GoalDescription::AtomicFormula(af).is_empty());

        let literal = TermLiteral::new(AtomicFormula::predicate(
            Predicate::string("x"),
            Vec::<Term>::new(),
        ));
        check!(!GoalDescription::Literal(literal).is_empty());

        let gd = GoalDescription::AtomicFormula(AtomicFormula::predicate(
            Predicate::string("on"),
            Vec::<Term>::new(),
        ));
        check!(!GoalDescription::Not(Box::new(gd.clone())).is_empty());
        check!(!GoalDescription::Imply(Box::new(gd.clone()), Box::new(gd.clone())).is_empty());
        check!(
            !GoalDescription::Exists(TypedVariables::default(), Box::new(gd.clone())).is_empty()
        );
        check!(
            !GoalDescription::ForAll(TypedVariables::default(), Box::new(gd.clone())).is_empty()
        );

        let f_comp = FluentComparison::new(
            BinaryComparison::GreaterThan,
            FluentExpression::number(Number::from(1)),
            FluentExpression::number(Number::from(0)),
        );
        check!(!GoalDescription::FluentComparison(f_comp).is_empty());
    }

    #[test]
    fn is_empty_recursive() {
        let empty_and = GoalDescription::and(Vec::new());
        check!(empty_and.is_empty());

        let empty_or = GoalDescription::or(Vec::new());
        check!(empty_or.is_empty());

        let nested_empty = GoalDescription::Not(Box::new(GoalDescription::and(Vec::new())));
        check!(nested_empty.is_empty());

        let non_empty_and = GoalDescription::and(vec![GoalDescription::AtomicFormula(
            AtomicFormula::predicate(Predicate::string("at"), Vec::<Term>::new()),
        )]);
        check!(!non_empty_and.is_empty());
    }
}
