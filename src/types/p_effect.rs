//! Contains primitive effects.

use crate::types::{AssignOp, AtomicFormula, FluentExpression, FunctionHead, FunctionTerm, Term};

/// A primitive effect. Occurs as part of a [`ConditionalEffect`](crate::types::ConditionalEffect) (within an [`Effect`](crate::types::Effects))
/// or a [`ConditionalEffect`](crate::types::ConditionalEffect).
///
/// ## Usage
/// Used by [`ConditionalEffect`](crate::ConditionalEffect) and [`ConditionalEffect`](crate::ConditionalEffect).
#[doc(alias("p-effect"))]
#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveEffect {
    AtomicFormula(AtomicFormula<Term>),
    NotAtomicFormula(AtomicFormula<Term>),
    /// ## Requirements
    /// Requires [Numeric Fluents](crate::Requirement::NumericFluents).
    AssignNumericFluent(AssignOp, FunctionHead, FluentExpression),
    /// ## Requirements
    /// Requires [Object Fluents](crate::Requirement::ObjectFluents).
    AssignObjectFluent(FunctionTerm, Option<Term>),
}

impl PrimitiveEffect {
    pub const fn new(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    pub const fn new_not(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }

    pub const fn new_numeric_fluent(
        op: AssignOp,
        head: FunctionHead,
        exp: FluentExpression,
    ) -> Self {
        Self::AssignNumericFluent(op, head, exp)
    }

    pub const fn new_object_fluent(f_term: FunctionTerm, term: Option<Term>) -> Self {
        Self::AssignObjectFluent(f_term, term)
    }
}

/// Alias for [`PrimitiveEffect`]; matches BNF `<p-effect>`.
#[deprecated(since = "0.2.0", note = "Use `PrimitiveEffect` instead")]
pub type PEffect = PrimitiveEffect;
