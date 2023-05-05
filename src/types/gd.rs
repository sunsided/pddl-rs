//! Contains goal definitions via the [`GoalDefinition`] type.

use crate::types::TermLiteral;
use crate::types::{AtomicFormula, FComp, Term, TypedVariables};

/// A goal definition.
///
/// ## Usage
/// Used by [`GD`] itself, as well as [`PreferenceGD`](crate::PreferenceGD), [`CEffect`](crate::CEffect),
/// [`TimedGD`](crate::TimedGD), [`DerivedPredicate`](crate::DerivedPredicate) and
/// [`Con2GD`](crate::Con2GD).
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
    FComp(FComp),
}

impl GoalDefinition {
    #[inline(always)]
    pub const fn new_atomic_formula(value: AtomicFormula<Term>) -> Self {
        Self::AtomicFormula(value)
    }

    #[inline(always)]
    pub const fn new_literal(value: TermLiteral) -> Self {
        Self::Literal(value)
    }

    #[inline(always)]
    pub fn new_and<T: IntoIterator<Item = GoalDefinition>>(values: T) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(values.into_iter().collect())
    }

    #[inline(always)]
    pub fn new_or<T: IntoIterator<Item = GoalDefinition>>(values: T) -> Self {
        // TODO: Flatten `(or (or a b) (or x y))` into `(or a b c y)`.
        Self::Or(values.into_iter().collect())
    }

    #[inline(always)]
    pub fn new_not(value: GoalDefinition) -> Self {
        Self::Not(Box::new(value))
    }

    #[inline(always)]
    pub fn new_imply_tuple(tuple: (GoalDefinition, GoalDefinition)) -> Self {
        Self::new_imply(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_imply(a: GoalDefinition, b: GoalDefinition) -> Self {
        Self::Imply(Box::new(a), Box::new(b))
    }

    #[inline(always)]
    pub fn new_exists_tuple(tuple: (TypedVariables, GoalDefinition)) -> Self {
        Self::new_exists(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_exists(variables: TypedVariables, gd: GoalDefinition) -> Self {
        Self::Exists(variables, Box::new(gd))
    }

    #[inline(always)]
    pub fn new_forall_tuple(tuple: (TypedVariables, GoalDefinition)) -> Self {
        Self::new_forall(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_forall(variables: TypedVariables, gd: GoalDefinition) -> Self {
        Self::ForAll(variables, Box::new(gd))
    }

    #[inline(always)]
    pub const fn new_f_comp(f_comp: FComp) -> Self {
        Self::FComp(f_comp)
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
            GoalDefinition::FComp(_) => false,
        }
    }
}
