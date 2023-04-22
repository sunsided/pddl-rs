//! Contains goal definitions.

use crate::types::{AtomicFormula, FComp, Literal, Term, TypedList, Variable};

/// A goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum GoalDefinition<'a> {
    AtomicFormula(AtomicFormula<'a, Term<'a>>),
    /// Requires [NegativePreconditions](crate::types::Requirement::NegativePreconditions).
    Literal(Literal<'a, Term<'a>>),
    And(Vec<GoalDefinition<'a>>),
    /// Requires [DisjunctivePreconditions](crate::types::Requirement::DisjunctivePreconditions).
    Or(Vec<GoalDefinition<'a>>),
    /// Requires [DisjunctivePreconditions](crate::types::Requirement::DisjunctivePreconditions).
    Not(Box<GoalDefinition<'a>>),
    /// Requires [DisjunctivePreconditions](crate::types::Requirement::DisjunctivePreconditions).
    Imply(Box<GoalDefinition<'a>>, Box<GoalDefinition<'a>>),
    /// Requires [ExistentialPreconditions](crate::types::Requirement::ExistentialPreconditions).
    Exists(TypedList<'a, Variable<'a>>, Box<GoalDefinition<'a>>),
    /// Requires [UniversalPreconditions](crate::types::Requirement::UniversalPreconditions).
    ForAll(TypedList<'a, Variable<'a>>, Box<GoalDefinition<'a>>),
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    FComp(FComp<'a>),
}

impl<'a> GoalDefinition<'a> {
    #[inline(always)]
    pub const fn new_atomic_formula(value: AtomicFormula<'a, Term<'a>>) -> Self {
        Self::AtomicFormula(value)
    }

    #[inline(always)]
    pub const fn new_literal(value: Literal<'a, Term<'a>>) -> Self {
        Self::Literal(value)
    }

    #[inline(always)]
    pub fn new_and<T: IntoIterator<Item = GoalDefinition<'a>>>(values: T) -> Self {
        Self::And(values.into_iter().collect())
    }

    #[inline(always)]
    pub fn new_or<T: IntoIterator<Item = GoalDefinition<'a>>>(values: T) -> Self {
        Self::Or(values.into_iter().collect())
    }

    #[inline(always)]
    pub fn new_not(value: GoalDefinition<'a>) -> Self {
        Self::Not(Box::new(value))
    }

    #[inline(always)]
    pub fn new_imply_tuple(tuple: (GoalDefinition<'a>, GoalDefinition<'a>)) -> Self {
        Self::new_imply(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_imply(a: GoalDefinition<'a>, b: GoalDefinition<'a>) -> Self {
        Self::Imply(Box::new(a), Box::new(b))
    }

    #[inline(always)]
    pub fn new_exists_tuple(tuple: (TypedList<'a, Variable<'a>>, GoalDefinition<'a>)) -> Self {
        Self::new_exists(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_exists(variables: TypedList<'a, Variable<'a>>, gd: GoalDefinition<'a>) -> Self {
        Self::Exists(variables, Box::new(gd))
    }

    #[inline(always)]
    pub fn new_forall_tuple(tuple: (TypedList<'a, Variable<'a>>, GoalDefinition<'a>)) -> Self {
        Self::new_forall(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_forall(variables: TypedList<'a, Variable<'a>>, gd: GoalDefinition<'a>) -> Self {
        Self::ForAll(variables, Box::new(gd))
    }

    #[inline(always)]
    pub const fn new_f_comp(f_comp: FComp<'a>) -> Self {
        Self::FComp(f_comp)
    }
}
