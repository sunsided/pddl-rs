use std::ops::Deref;

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    Constants, Functions, GoalDef, InitElement, InitElements, LengthSpec, MetricSpec, Objects,
    PredicateDefinitions, Requirements, Types,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Requirements {}
impl sealed::Sealed for Types {}
impl sealed::Sealed for Constants {}
impl sealed::Sealed for PredicateDefinitions {}
impl sealed::Sealed for Functions {}
impl sealed::Sealed for Objects {}
impl sealed::Sealed for InitElements {}
impl sealed::Sealed for GoalDef {}
impl sealed::Sealed for MetricSpec {}
impl sealed::Sealed for LengthSpec {}
impl sealed::Sealed for InitElement {}

impl<'a> Visitor<Requirements, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Requirements) -> RcDoc<'a> {
        RcDoc::intersperse(value.iter().map(|r| r.accept(self)), RcDoc::softline())
    }
}

impl<'a> Visitor<Types, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Types) -> RcDoc<'a> {
        value.values().accept(self)
    }
}

impl<'a> Visitor<Constants, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Constants) -> RcDoc<'a> {
        value.deref().accept(self)
    }
}

impl<'a> Visitor<PredicateDefinitions, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &PredicateDefinitions) -> RcDoc<'a> {
        RcDoc::intersperse(
            value.values().iter().map(|p| p.accept(self)),
            RcDoc::softline(),
        )
    }
}

impl<'a> Visitor<Functions, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Functions) -> RcDoc<'a> {
        value.values().accept(self)
    }
}

impl<'a> Visitor<Objects, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Objects) -> RcDoc<'a> {
        value.values().accept(self)
    }
}

impl<'a> Visitor<InitElements, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &InitElements) -> RcDoc<'a> {
        RcDoc::intersperse(
            value.values().iter().map(|ie| ie.accept(self)),
            RcDoc::softline(),
        )
    }
}

impl<'a> Visitor<InitElement, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &InitElement) -> RcDoc<'a> {
        match value {
            InitElement::Literal(lit) => lit.accept(self),
            InitElement::At(n, lit) => RcDoc::text("(at ")
                .append(n.accept(self))
                .append(" ")
                .append(lit.accept(self))
                .append(")"),
            InitElement::IsValue(term, n) => RcDoc::text("(= ")
                .append(term.accept(self))
                .append(" ")
                .append(n.accept(self))
                .append(")"),
            InitElement::IsObject(term, name) => RcDoc::text("(= ")
                .append(term.accept(self))
                .append(" ")
                .append(name.accept(self))
                .append(")"),
        }
    }
}

impl<'a> Visitor<GoalDef, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &GoalDef) -> RcDoc<'a> {
        value.deref().accept(self)
    }
}

impl<'a> Visitor<MetricSpec, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &MetricSpec) -> RcDoc<'a> {
        RcDoc::text("(")
            .append(value.optimization().accept(self))
            .append(" ")
            .append(value.expression().accept(self))
            .append(")")
    }
}

impl<'a> Visitor<LengthSpec, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &LengthSpec) -> RcDoc<'a> {
        let mut doc = RcDoc::text("(:length");
        if let Some(s) = value.serial() {
            doc = doc.append(RcDoc::text(format!(" :serial {s}")));
        }
        if let Some(p) = value.parallel() {
            doc = doc.append(RcDoc::text(format!(" :parallel {p}")));
        }
        doc.append(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn requirements_works() {
        let r = crate::Requirements::new([crate::Requirement::Strips, crate::Requirement::Typing]);
        assert_eq!(prettify!(r, 20), ":strips :typing");
        assert_eq!(prettify!(r, 10), ":strips\n:typing");
    }

    #[test]
    fn metric_spec_works() {
        let ms = crate::MetricSpec::new(
            crate::Optimization::Minimize,
            crate::MetricFExp::new_total_time(),
        );
        assert_eq!(prettify!(ms, 30), "(minimize total-time)");
    }

    #[test]
    fn length_spec_works() {
        let ls = crate::LengthSpec::new_serial(10);
        assert_eq!(prettify!(ls, 30), "(:length :serial 10)");
    }
}
