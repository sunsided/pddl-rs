//! Contains p-effects.

use crate::types::{AssignOp, AtomicFormula, FExp, FHead, FunctionTerm, Term};

/// A p-effect. Occurs as part of a [`CEffect`](crate::types::CEffect) (within an [`Effect`](crate::types::Effects))
/// or a [`ConditionalEffect`](crate::types::ConditionalEffect).
///
/// ## Usage
/// Used by [`CEffect`](crate::CEffect) and [`ConditionalEffect`](crate::ConditionalEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum PEffect {
    AtomicFormula(AtomicFormula<Term>),
    NotAtomicFormula(AtomicFormula<Term>),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    AssignNumericFluent(AssignOp, FHead, FExp),
    /// ## Requirements
    /// Requires [Object Fluents](crate::Requirement::ObjectFluents).
    AssignObjectFluent(FunctionTerm, Option<Term>),
}

impl PEffect {
    pub const fn new(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    pub const fn new_not(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }

    pub const fn new_numeric_fluent(op: AssignOp, head: FHead, exp: FExp) -> Self {
        Self::AssignNumericFluent(op, head, exp)
    }

    pub const fn new_object_fluent(f_term: FunctionTerm, term: Option<Term>) -> Self {
        Self::AssignObjectFluent(f_term, term)
    }
}
