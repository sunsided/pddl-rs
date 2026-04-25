use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::Timeless;
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Timeless {}

impl Visitor<Timeless, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Timeless) -> RcDoc<'static> {
        if value.values().is_empty() {
            return RcDoc::nil();
        }
        RcDoc::text("(:timeless")
            .append(RcDoc::softline())
            .append(RcDoc::intersperse(
                value.values().iter().map(|lit| lit.accept(self)),
                RcDoc::softline(),
            ))
            .nest(4)
            .group()
            .append(")")
    }
}
