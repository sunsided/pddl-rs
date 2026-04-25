use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{ConditionalEffect, EffectCondition, Effects, ForallConditionalEffect, PrimitiveEffect, WhenConditionalEffect};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for PrimitiveEffect {}
impl sealed::Sealed for ConditionalEffect {}
impl sealed::Sealed for EffectCondition {}
impl sealed::Sealed for ForallConditionalEffect {}
impl sealed::Sealed for WhenConditionalEffect {}
impl sealed::Sealed for Effects {}

impl Visitor<PrimitiveEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PrimitiveEffect) -> RcDoc<'static> {
        match value {
            PrimitiveEffect::AtomicFormula(af) => af.accept(self),
            PrimitiveEffect::NotAtomicFormula(af) => {
                RcDoc::text("(not ").append(af.accept(self)).append(")")
            }
            PrimitiveEffect::AssignNumericFluent(op, head, exp) => RcDoc::text("(")
                .append(op.accept(self))
                .append(" ")
                .append(head.accept(self))
                .append(" ")
                .append(exp.accept(self))
                .append(")"),
            PrimitiveEffect::AssignObjectFluent(ft, term) => match term {
                Some(t) => RcDoc::text("(assign ")
                    .append(ft.accept(self))
                    .append(" ")
                    .append(t.accept(self))
                    .append(")"),
                None => RcDoc::text("(assign ").append(ft.accept(self)).append(")"),
            },
        }
    }
}

impl Visitor<ConditionalEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ConditionalEffect) -> RcDoc<'static> {
        match value {
            ConditionalEffect::Effect(pe) => pe.accept(self),
            ConditionalEffect::Forall(fc) => fc.accept(self),
            ConditionalEffect::When(wc) => wc.accept(self),
        }
    }
}

impl Visitor<ForallConditionalEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ForallConditionalEffect) -> RcDoc<'static> {
        RcDoc::text("(forall (")
            .append(value.variables.accept(self))
            .append(") ")
            .append(value.effects.accept(self))
            .append(")")
    }
}

impl Visitor<WhenConditionalEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &WhenConditionalEffect) -> RcDoc<'static> {
        RcDoc::text("(when ")
            .append(value.condition.accept(self))
            .append(" ")
            .append(value.effect.accept(self))
            .append(")")
    }
}

impl Visitor<EffectCondition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &EffectCondition) -> RcDoc<'static> {
        match value {
            EffectCondition::Single(pe) => pe.accept(self),
            EffectCondition::All(pes) => {
                if pes.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    RcDoc::text("(and")
                        .append(RcDoc::softline())
                        .append(RcDoc::intersperse(
                            pes.iter().map(|pe| pe.accept(self)),
                            RcDoc::softline(),
                        ))
                        .nest(4)
                        .group()
                        .append(")")
                }
            }
        }
    }
}

impl Visitor<Effects, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Effects) -> RcDoc<'static> {
        RcDoc::text("(and")
            .append(RcDoc::softline())
            .append(RcDoc::intersperse(
                value.iter().map(|ce| ce.accept(self)),
                RcDoc::softline(),
            ))
            .nest(4)
            .group()
            .append(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{
        AtomicFormula, ConditionalEffect, EffectCondition, FunctionSymbol, GoalDefinition, Name, PrimitiveEffect, Predicate,
        Term,
    };

    #[test]
    fn p_effect_atomic() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_name(Name::new("B")),
                Term::new_name(Name::new("home")),
            ],
        );
        let pe = PrimitiveEffect::new(af);
        assert_eq!(prettify!(pe, 30), "(at B home)");
    }

    #[test]
    fn p_effect_not() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("B"))],
        );
        let pe = PrimitiveEffect::new_not(af);
        assert_eq!(prettify!(pe, 30), "(not (at B))");
    }

    #[test]
    fn p_effect_assign() {
        let pe = PrimitiveEffect::new_numeric_fluent(
            crate::AssignOp::Assign,
            crate::FunctionHead::new(FunctionSymbol::from("battery")),
            crate::FluentExpression::new_number(10),
        );
        assert_eq!(prettify!(pe, 30), "(assign battery 10)");
    }

    #[test]
    fn effects_single() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("B"))],
        );
        let pe = PrimitiveEffect::new(af);
        let ce = ConditionalEffect::new_primitive_effect(pe);
        let effects = Effects::new(ce);
        assert_eq!(prettify!(effects, 30), "(and (at B))");
    }

    #[test]
    fn p_effect_assign_object_fluent_with_term() {
        use crate::{FunctionSymbol, FunctionTerm};
        let ft = FunctionTerm::new(
            FunctionSymbol::from("loc"),
            vec![Term::new_name(Name::new("x"))],
        );
        let pe = PrimitiveEffect::new_object_fluent(ft, Some(Term::new_name(Name::new("y"))));
        assert_eq!(prettify!(pe, 30), "(assign (loc x) y)");
    }

    #[test]
    fn p_effect_assign_object_fluent_without_term() {
        use crate::{FunctionSymbol, FunctionTerm};
        let ft = FunctionTerm::new(FunctionSymbol::from("loc"), vec![]);
        let pe = PrimitiveEffect::new_object_fluent(ft, None);
        assert_eq!(prettify!(pe, 30), "(assign loc)");
    }

    #[test]
    fn c_effect_forall() {
        use crate::{ToTyped, Type, TypedVariables, Variable};
        let vars: TypedVariables = vec![Variable::new_string("x").to_typed(Type::OBJECT)].into();
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("p"),
            vec![Term::new_variable(Variable::from("x"))],
        );
        let pe = PrimitiveEffect::new(af);
        let effects = Effects::new(ConditionalEffect::new_primitive_effect(pe));
        let ce = ConditionalEffect::new_forall(vars, effects);
        assert_eq!(prettify!(ce, 40), "(forall (?x) (and (p ?x)))");
    }

    #[test]
    fn c_effect_when() {
        let cond = GoalDefinition::new_and(Vec::<GoalDefinition>::new());
        let ce_inner = EffectCondition::new_and(Vec::new());
        let ce = ConditionalEffect::new_when(cond, ce_inner);
        assert_eq!(prettify!(ce, 30), "(when (and) (and))");
    }

    #[test]
    fn conditional_effect_all_empty() {
        let ce = EffectCondition::new_and(Vec::new());
        assert_eq!(prettify!(ce, 20), "(and)");
    }

    #[test]
    fn conditional_effect_all_non_empty() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("x"))],
        );
        let pe1 = PrimitiveEffect::new(af.clone());
        let pe2 = PrimitiveEffect::new(af);
        let ce = EffectCondition::new_and(vec![pe1, pe2]);
        assert_eq!(prettify!(ce, 30), "(and (at x) (at x))");
    }

    #[test]
    fn effects_multi_effect() {
        let af1 = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("x"))],
        );
        let af2 = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("y"))],
        );
        let ce1 = ConditionalEffect::new_primitive_effect(PrimitiveEffect::new(af1));
        let ce2 = ConditionalEffect::new_primitive_effect(PrimitiveEffect::new(af2));
        let effects = Effects::new_and(vec![ce1, ce2]);
        assert_eq!(prettify!(effects, 40), "(and (at x) (at y))");
    }
}
