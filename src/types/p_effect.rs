//! Contains primitive effects.

use crate::types::{AssignOp, AtomicFormula, FluentExpression, FunctionHead, FunctionTerm, Term};

/// A primitive effect. Occurs as part of a [`ConditionalEffect`](crate::ConditionalEffect) via its `Effect` variant,
/// or nested within an [`EffectCondition`](crate::EffectCondition) inside `when` clauses.
///
/// ## Usage
/// Used by [`ConditionalEffect`](crate::ConditionalEffect) and [`EffectCondition`](crate::EffectCondition).
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

    pub const fn atomic_formula(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::AtomicFormula(atomic_formula)
    }

    #[deprecated(since = "0.2.0", note = "Use `atomic_formula` instead")]
    pub const fn new_atomic_formula(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::atomic_formula(atomic_formula)
    }

    pub const fn not(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::NotAtomicFormula(atomic_formula)
    }

    #[deprecated(since = "0.2.0", note = "Use `not` instead")]
    pub const fn new_not(atomic_formula: AtomicFormula<Term>) -> Self {
        Self::not(atomic_formula)
    }

    pub const fn numeric_fluent(op: AssignOp, head: FunctionHead, exp: FluentExpression) -> Self {
        Self::AssignNumericFluent(op, head, exp)
    }

    #[deprecated(since = "0.2.0", note = "Use `numeric_fluent` instead")]
    pub const fn new_numeric_fluent(
        op: AssignOp,
        head: FunctionHead,
        exp: FluentExpression,
    ) -> Self {
        Self::numeric_fluent(op, head, exp)
    }

    pub const fn object_fluent(f_term: FunctionTerm, term: Option<Term>) -> Self {
        Self::AssignObjectFluent(f_term, term)
    }

    #[deprecated(since = "0.2.0", note = "Use `object_fluent` instead")]
    pub const fn new_object_fluent(f_term: FunctionTerm, term: Option<Term>) -> Self {
        Self::object_fluent(f_term, term)
    }
}

/// Alias for [`PrimitiveEffect`]; matches BNF `<p-effect>`.
#[deprecated(since = "0.2.0", note = "Use `PrimitiveEffect` instead")]
pub type PEffect = PrimitiveEffect;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{FunctionSymbol, Name};
    use crate::Predicate;

    fn make_atomic_formula() -> AtomicFormula<Term> {
        AtomicFormula::predicate(
            Predicate::string("on"),
            vec![
                Term::new_name(Name::new("a")),
                Term::new_name(Name::new("b")),
            ],
        )
    }

    #[test]
    fn atomic_formula_new() {
        let af = make_atomic_formula();
        let pe = PrimitiveEffect::new(af.clone());
        assert_eq!(pe, PrimitiveEffect::AtomicFormula(af));
    }

    #[test]
    fn atomic_formula_constructor() {
        let af = make_atomic_formula();
        let pe = PrimitiveEffect::atomic_formula(af.clone());
        assert_eq!(pe, PrimitiveEffect::AtomicFormula(af));
    }

    #[test]
    #[allow(deprecated)]
    fn atomic_formula_deprecated_new() {
        let af = make_atomic_formula();
        let pe = PrimitiveEffect::new_atomic_formula(af.clone());
        assert_eq!(pe, PrimitiveEffect::AtomicFormula(af));
    }

    #[test]
    fn not_constructor() {
        let af = make_atomic_formula();
        let pe = PrimitiveEffect::not(af.clone());
        assert_eq!(pe, PrimitiveEffect::NotAtomicFormula(af));
    }

    #[test]
    #[allow(deprecated)]
    fn not_deprecated_new() {
        let af = make_atomic_formula();
        let pe = PrimitiveEffect::new_not(af.clone());
        assert_eq!(pe, PrimitiveEffect::NotAtomicFormula(af));
    }

    #[test]
    fn numeric_fluent_constructor() {
        let head = FunctionHead::Simple(FunctionSymbol::string("total-cost"));
        let exp = FluentExpression::number(10);
        let pe = PrimitiveEffect::numeric_fluent(AssignOp::Assign, head, exp);
        assert!(matches!(pe, PrimitiveEffect::AssignNumericFluent(_, _, _)));
    }

    #[test]
    #[allow(deprecated)]
    fn numeric_fluent_deprecated_new() {
        let head = FunctionHead::Simple(FunctionSymbol::string("total-cost"));
        let exp = FluentExpression::number(10);
        let pe = PrimitiveEffect::new_numeric_fluent(AssignOp::Assign, head, exp);
        assert!(matches!(pe, PrimitiveEffect::AssignNumericFluent(_, _, _)));
    }

    #[test]
    fn object_fluent_with_value() {
        let f_term = FunctionTerm::new(FunctionSymbol::string("loc"), vec![]);
        let term = Term::new_name(Name::new("home"));
        let pe = PrimitiveEffect::object_fluent(f_term.clone(), Some(term.clone()));
        assert_eq!(pe, PrimitiveEffect::AssignObjectFluent(f_term, Some(term)));
    }

    #[test]
    fn object_fluent_without_value() {
        let f_term = FunctionTerm::new(FunctionSymbol::string("loc"), vec![]);
        let pe = PrimitiveEffect::object_fluent(f_term.clone(), None);
        assert_eq!(pe, PrimitiveEffect::AssignObjectFluent(f_term, None));
    }

    #[test]
    #[allow(deprecated)]
    fn object_fluent_deprecated_new() {
        let f_term = FunctionTerm::new(FunctionSymbol::string("loc"), vec![]);
        let pe = PrimitiveEffect::new_object_fluent(f_term.clone(), None);
        assert_eq!(pe, PrimitiveEffect::AssignObjectFluent(f_term, None));
    }

    #[test]
    fn clone_works() {
        let af = make_atomic_formula();
        let pe = PrimitiveEffect::atomic_formula(af);
        let clone = pe.clone();
        assert_eq!(pe, clone);
    }

    #[test]
    fn debug_impl() {
        let af = make_atomic_formula();
        let pe = PrimitiveEffect::atomic_formula(af);
        let dbg = format!("{pe:?}");
        assert!(dbg.contains("AtomicFormula"));
    }

    #[test]
    fn all_assign_ops_numeric_fluent() {
        let head = FunctionHead::Simple(FunctionSymbol::string("f"));
        let exp = FluentExpression::number(1);
        for op in [
            AssignOp::Assign,
            AssignOp::ScaleUp,
            AssignOp::ScaleDown,
            AssignOp::Increase,
            AssignOp::Decrease,
        ] {
            let pe = PrimitiveEffect::numeric_fluent(op, head.clone(), exp.clone());
            assert!(matches!(pe, PrimitiveEffect::AssignNumericFluent(_, _, _)));
        }
    }

    #[test]
    #[allow(deprecated)]
    fn deprecated_alias_exists() {
        fn _assert_alias(_: PEffect) {}
    }
}
