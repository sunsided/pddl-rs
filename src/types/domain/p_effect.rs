//! Contains p-effects.

use crate::types::domain::{AssignOp, AtomicFormula, FExp, FHead, FunctionTerm, Term};

/// A p-effect.
#[derive(Debug, Clone, PartialEq)]
pub enum PEffect<'a> {
    AtomicFormula(AtomicFormula<'a, Term<'a>>),
    NotAtomicFormula(AtomicFormula<'a, Term<'a>>),
    /// Requires [NumericFluents](crate::types::Requirement::NumericFluents).
    AssignNumericFluent(AssignOp, FHead<'a>, FExp<'a>),
    /// Requires [ObjectFluents](crate::types::Requirement::ObjectFluents).
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
