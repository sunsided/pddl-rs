use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{CEffect, ConditionalEffect, Effects, ForallCEffect, PEffect, WhenCEffect};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for PEffect {}
impl sealed::Sealed for CEffect {}
impl sealed::Sealed for ForallCEffect {}
impl sealed::Sealed for WhenCEffect {}
impl sealed::Sealed for ConditionalEffect {}
impl sealed::Sealed for Effects {}

impl Visitor<PEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PEffect) -> RcDoc<'static> {
        match value {
            PEffect::AtomicFormula(af) => af.accept(self),
            PEffect::NotAtomicFormula(af) => {
                RcDoc::text("(not ").append(af.accept(self)).append(")")
            }
            PEffect::AssignNumericFluent(op, head, exp) => RcDoc::text("(")
                .append(op.accept(self))
                .append(" ")
                .append(head.accept(self))
                .append(" ")
                .append(exp.accept(self))
                .append(")"),
            PEffect::AssignObjectFluent(ft, term) => match term {
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

impl Visitor<CEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &CEffect) -> RcDoc<'static> {
        match value {
            CEffect::Effect(pe) => pe.accept(self),
            CEffect::Forall(fc) => fc.accept(self),
            CEffect::When(wc) => wc.accept(self),
        }
    }
}

impl Visitor<ForallCEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ForallCEffect) -> RcDoc<'static> {
        RcDoc::text("(forall (")
            .append(value.variables.accept(self))
            .append(") ")
            .append(value.effects.accept(self))
            .append(")")
    }
}

impl Visitor<WhenCEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &WhenCEffect) -> RcDoc<'static> {
        RcDoc::text("(when ")
            .append(value.condition.accept(self))
            .append(" ")
            .append(value.effect.accept(self))
            .append(")")
    }
}

impl Visitor<ConditionalEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ConditionalEffect) -> RcDoc<'static> {
        match value {
            ConditionalEffect::Single(pe) => pe.accept(self),
            ConditionalEffect::All(pes) => {
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
    use crate::{AtomicFormula, FunctionSymbol, Name, Predicate, Term};

    #[test]
    fn p_effect_atomic() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_name(Name::new("B")),
                Term::new_name(Name::new("home")),
            ],
        );
        let pe = PEffect::new(af);
        assert_eq!(prettify!(pe, 30), "(at B home)");
    }

    #[test]
    fn p_effect_not() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("B"))],
        );
        let pe = PEffect::new_not(af);
        assert_eq!(prettify!(pe, 30), "(not (at B))");
    }

    #[test]
    fn p_effect_assign() {
        let pe = PEffect::new_numeric_fluent(
            crate::AssignOp::Assign,
            crate::FHead::new(FunctionSymbol::from("battery")),
            crate::FExp::new_number(10),
        );
        assert_eq!(prettify!(pe, 30), "(assign battery 10)");
    }

    #[test]
    fn effects_single() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("B"))],
        );
        let pe = PEffect::new(af);
        let ce = CEffect::new_p_effect(pe);
        let effects = Effects::new(ce);
        assert_eq!(prettify!(effects, 30), "(and (at B))");
    }
}
