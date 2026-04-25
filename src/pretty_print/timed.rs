use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{DurationConstraint, DurationValue, SimpleDurationConstraint};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for DurationValue {}
impl sealed::Sealed for SimpleDurationConstraint {}
impl sealed::Sealed for DurationConstraint {}

impl<'a> Visitor<DurationValue, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &DurationValue) -> RcDoc<'a> {
        match value {
            DurationValue::Number(n) => n.accept(self),
            DurationValue::FExp(e) => e.accept(self),
        }
    }
}

impl<'a> Visitor<SimpleDurationConstraint, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &SimpleDurationConstraint) -> RcDoc<'a> {
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

impl<'a> Visitor<DurationConstraint, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &DurationConstraint) -> RcDoc<'a> {
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
