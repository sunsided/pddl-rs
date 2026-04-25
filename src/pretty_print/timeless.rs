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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn timeless_empty_works() {
        let t = Timeless::default();
        assert_eq!(prettify!(t, 20), "");
    }

    #[test]
    fn timeless_non_empty_works() {
        use crate::{AtomicFormula, Literal, Name};
        let af =
            AtomicFormula::new_predicate(crate::Predicate::new_string("p"), vec![Name::new("x")]);
        let t = Timeless::new(vec![Literal::new(af)]);
        assert_eq!(prettify!(t, 20), "(:timeless (p x))");
    }
}
