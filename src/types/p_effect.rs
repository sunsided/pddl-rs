//! Contains p-effects.

use crate::types::{AssignOp, AtomicFormula, FExp, FHead, FunctionTerm, Term};

/// A p-effect. Occurs as part of a [`CEffect`](crate::types::CEffect) (within an [`Effect`](crate::types::Effect))
/// or a [`ConditionalEffect`](crate::types::ConditionalEffect).
///
/// ## Usage
/// Used by [`CEffect`](crate::CEffect) and [`ConditionalEffect`](crate::ConditionalEffect).
#[derive(Debug, Clone, PartialEq)]
pub enum PEffect<'a> {
    AtomicFormula(AtomicFormula<'a, Term<'a>>),
    NotAtomicFormula(AtomicFormula<'a, Term<'a>>),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    AssignNumericFluent(AssignOp, FHead<'a>, FExp<'a>),
    /// ## Requirements
    /// Requires [Object Fluents](crate::Requirement::ObjectFluents).
    AssignObjectFluent(FunctionTerm<'a>, Option<Term<'a>>),
}

impl<'a> PEffect<'a> {
    pub const fn new(atomic_formula: AtomicFormula<'a, Term<'a>>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    pub const fn new_not(atomic_formula: AtomicFormula<'a, Term<'a>>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }

    pub const fn new_numeric_fluent(op: AssignOp, head: FHead<'a>, exp: FExp<'a>) -> Self {
        Self::AssignNumericFluent(op, head, exp)
    }

    pub const fn new_object_fluent(f_term: FunctionTerm<'a>, term: Option<Term<'a>>) -> Self {
        Self::AssignObjectFluent(f_term, term)
    }
}
