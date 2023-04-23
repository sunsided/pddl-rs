//! Contains goal definitions via the [`GoalDefinition`] type.

use crate::types::TermLiteral;
use crate::types::{AtomicFormula, FComp, Term, TypedVariables};

/// A goal definition.
#[derive(Debug, Clone, PartialEq)]
pub enum GoalDefinition<'a> {
    AtomicFormula(AtomicFormula<'a, Term<'a>>),
    /// Requires [NegativePreconditions](crate::types::Requirement::NegativePreconditions).
    Literal(TermLiteral<'a>),
    And(Vec<GoalDefinition<'a>>),
    /// Requires [DisjunctivePreconditions](crate::types::Requirement::DisjunctivePreconditions).
    Or(Vec<GoalDefinition<'a>>),
    /// Requires [DisjunctivePreconditions](crate::types::Requirement::DisjunctivePreconditions).
    Not(Box<GoalDefinition<'a>>),
    /// Requires [DisjunctivePreconditions](crate::types::Requirement::DisjunctivePreconditions).
    Imply(Box<GoalDefinition<'a>>, Box<GoalDefinition<'a>>),
    /// Requires [ExistentialPreconditions](crate::types::Requirement::ExistentialPreconditions).
    Exists(TypedVariables<'a>, Box<GoalDefinition<'a>>),
    /// Requires [UniversalPreconditions](crate::types::Requirement::UniversalPreconditions).
    ForAll(TypedVariables<'a>, Box<GoalDefinition<'a>>),
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    FComp(FComp<'a>),
}

impl<'a> GoalDefinition<'a> {
    #[inline(always)]
    pub const fn new_atomic_formula(value: AtomicFormula<'a, Term<'a>>) -> Self {
        Self::AtomicFormula(value)
    }

    #[inline(always)]
    pub const fn new_literal(value: TermLiteral<'a>) -> Self {
        Self::Literal(value)
    }

    #[inline(always)]
    pub fn new_and<T: IntoIterator<Item = GoalDefinition<'a>>>(values: T) -> Self {
        // TODO: Flatten `(and (and a b) (and x y))` into `(and a b c y)`.
        Self::And(values.into_iter().collect())
    }

    #[inline(always)]
    pub fn new_or<T: IntoIterator<Item = GoalDefinition<'a>>>(values: T) -> Self {
        // TODO: Flatten `(or (or a b) (or x y))` into `(or a b c y)`.
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
    pub fn new_exists_tuple(tuple: (TypedVariables<'a>, GoalDefinition<'a>)) -> Self {
        Self::new_exists(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_exists(variables: TypedVariables<'a>, gd: GoalDefinition<'a>) -> Self {
        Self::Exists(variables, Box::new(gd))
    }

    #[inline(always)]
    pub fn new_forall_tuple(tuple: (TypedVariables<'a>, GoalDefinition<'a>)) -> Self {
        Self::new_forall(tuple.0, tuple.1)
    }

    #[inline(always)]
    pub fn new_forall(variables: TypedVariables<'a>, gd: GoalDefinition<'a>) -> Self {
        Self::ForAll(variables, Box::new(gd))
    }

    #[inline(always)]
    pub const fn new_f_comp(f_comp: FComp<'a>) -> Self {
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
