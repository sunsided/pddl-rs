use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    DurationConstraint, DurationValue, DurativeActionEffect, DurativeActionGoalDefinition,
    PrefTimedGD, SimpleDurationConstraint, TimedEffect, TimedGD,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for DurationValue {}
impl sealed::Sealed for SimpleDurationConstraint {}
impl sealed::Sealed for DurationConstraint {}
impl sealed::Sealed for TimedGD {}
impl sealed::Sealed for PrefTimedGD {}
impl sealed::Sealed for TimedEffect {}
impl sealed::Sealed for DurativeActionEffect {}
impl sealed::Sealed for DurativeActionGoalDefinition {}

impl Visitor<DurationValue, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurationValue) -> RcDoc<'static> {
        match value {
            DurationValue::Number(n) => n.accept(self),
            DurationValue::FExp(e) => e.accept(self),
        }
    }
}

impl Visitor<SimpleDurationConstraint, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &SimpleDurationConstraint) -> RcDoc<'static> {
        match value {
            SimpleDurationConstraint::Op(op, dv) => RcDoc::text("(")
                .append(op.accept(self))
                .append(" ?duration ")
                .append(dv.accept(self))
                .append(")"),
            SimpleDurationConstraint::At(ts, inner) => {
                let inner_doc = self.visit(inner.as_ref());
                RcDoc::text("(at ")
                    .append(ts.accept(self))
                    .append(" ")
                    .append(inner_doc)
                    .append(")")
            }
        }
    }
}

impl Visitor<DurationConstraint, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurationConstraint) -> RcDoc<'static> {
        match value {
            DurationConstraint::Single(sdc) => sdc.accept(self),
            DurationConstraint::All(sdcs) => RcDoc::text("(and")
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    sdcs.iter().map(|sdc| sdc.accept(self)),
                    RcDoc::softline(),
                ))
                .nest(4)
                .group()
                .append(")"),
        }
    }
}

impl Visitor<TimedGD, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &TimedGD) -> RcDoc<'static> {
        match value {
            TimedGD::At(ts, gd) => RcDoc::text("(at ")
                .append(ts.accept(self))
                .append(" ")
                .append(gd.accept(self))
                .append(")"),
            TimedGD::Over(interval, gd) => RcDoc::text("(over ")
                .append(interval.accept(self))
                .append(" ")
                .append(gd.accept(self))
                .append(")"),
        }
    }
}

impl Visitor<PrefTimedGD, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PrefTimedGD) -> RcDoc<'static> {
        match value {
            PrefTimedGD::Required(tgd) => tgd.accept(self),
            PrefTimedGD::Preference(name, tgd) => {
                let children: Vec<RcDoc<'static>> = match name {
                    Some(n) => vec![n.accept(self), self.visit(tgd)],
                    None => vec![self.visit(tgd)],
                };
                self.sexpr("preference", children)
            }
        }
    }
}

impl Visitor<TimedEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &TimedEffect) -> RcDoc<'static> {
        match value {
            TimedEffect::Conditional(ts, ce) => RcDoc::text("(at ")
                .append(ts.accept(self))
                .append(" ")
                .append(ce.accept(self))
                .append(")"),
            TimedEffect::NumericFluent(ts, fassign) => RcDoc::text("(at ")
                .append(ts.accept(self))
                .append(" ")
                .append(fassign.accept(self))
                .append(")"),
            TimedEffect::ContinuousEffect(op, head, exp) => RcDoc::text("(")
                .append(op.accept(self))
                .append(" ")
                .append(head.accept(self))
                .append(" ")
                .append(exp.accept(self))
                .append(")"),
        }
    }
}

impl Visitor<DurativeActionGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionGoalDefinition) -> RcDoc<'static> {
        match value {
            DurativeActionGoalDefinition::Timed(ptgd) => ptgd.accept(self),
            DurativeActionGoalDefinition::And(items) => {
                if items.is_empty() {
                    RcDoc::text("(and)")
                } else {
                    self.sexpr("and", items.iter().map(|dagd| self.visit(dagd)))
                }
            }
            DurativeActionGoalDefinition::Forall(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(body.as_ref()),
                ],
            ),
        }
    }
}

impl Visitor<DurativeActionEffect, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionEffect) -> RcDoc<'static> {
        match value {
            DurativeActionEffect::Timed(te) => te.accept(self),
            DurativeActionEffect::All(effects) => RcDoc::text("(and")
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    effects.iter().map(|e| self.visit(e)),
                    RcDoc::softline(),
                ))
                .nest(4)
                .group()
                .append(")"),
            DurativeActionEffect::Forall(vars, body) => self.sexpr(
                "forall",
                [
                    RcDoc::text("(").append(vars.accept(self)).append(")"),
                    self.visit(body.as_ref()),
                ],
            ),
            DurativeActionEffect::When(condition, effect) => RcDoc::text("(at ")
                .append(self.visit(condition))
                .append(" ")
                .append(self.visit(effect))
                .append(")"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn duration_value_number() {
        let dv = DurationValue::new_number(10);
        assert_eq!(prettify!(dv, 20), "10");
    }

    #[test]
    fn simple_duration_constraint_op() {
        let sdc =
            SimpleDurationConstraint::new_op(crate::DOp::Equal, DurationValue::new_number(10));
        assert_eq!(prettify!(sdc, 30), "(= ?duration 10)");
    }
}
