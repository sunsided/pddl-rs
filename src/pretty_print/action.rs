use std::ops::Deref;

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    ActionDefinition, DerivedPredicate, DurativeActionDefinition, StructureDef, StructureDefs,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for ActionDefinition {}
impl sealed::Sealed for DurativeActionDefinition {}
impl sealed::Sealed for DerivedPredicate {}
impl sealed::Sealed for StructureDef {}
impl sealed::Sealed for StructureDefs {}

impl Visitor<ActionDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ActionDefinition) -> RcDoc<'static> {
        let mut doc = RcDoc::text("(:action ")
            .append(value.symbol().accept(self))
            .append(RcDoc::hardline())
            .append(self.keyword_line("parameters"))
            .append(RcDoc::text(" ("))
            .append(value.parameters().accept(self))
            .append(")")
            .append(RcDoc::hardline())
            .append(self.keyword_line("precondition"))
            .append(RcDoc::text(" "))
            .append(value.precondition().accept(self));

        if let Some(effect) = value.effect() {
            doc = doc
                .append(RcDoc::hardline())
                .append(self.keyword_line("effect"))
                .append(RcDoc::text(" "))
                .append(effect.accept(self));
        }

        doc.append(")")
    }
}

impl Visitor<DurativeActionDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionDefinition) -> RcDoc<'static> {
        let mut doc = RcDoc::text("(:durative-action ")
            .append(value.symbol().accept(self))
            .append(RcDoc::hardline())
            .append(self.keyword_line("parameters"))
            .append(RcDoc::text(" ("))
            .append(value.parameters().accept(self))
            .append(")");

        if let Some(duration) = value.duration() {
            doc = doc
                .append(RcDoc::hardline())
                .append(self.keyword_line("duration"))
                .append(RcDoc::text(" "))
                .append(duration.accept(self));
        }

        if let Some(condition) = value.condition() {
            doc = doc
                .append(RcDoc::hardline())
                .append(self.keyword_line("condition"))
                .append(RcDoc::text(" "))
                .append(condition.accept(self));
        }

        if let Some(effect) = value.effect() {
            doc = doc
                .append(RcDoc::hardline())
                .append(self.keyword_line("effect"))
                .append(RcDoc::text(" "))
                .append(effect.accept(self));
        }

        doc.append(")")
    }
}

impl Visitor<DerivedPredicate, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DerivedPredicate) -> RcDoc<'static> {
        RcDoc::text("(:derived ")
            .append(value.predicate().accept(self))
            .append(" ")
            .append(value.expression().accept(self))
            .append(")")
    }
}

impl Visitor<StructureDef, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &StructureDef) -> RcDoc<'static> {
        match value {
            StructureDef::Action(a) => a.accept(self),
            StructureDef::DurativeAction(da) => da.as_ref().accept(self),
            StructureDef::Derived(d) => d.accept(self),
        }
    }
}

impl Visitor<StructureDefs, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &StructureDefs) -> RcDoc<'static> {
        RcDoc::intersperse(
            value.deref().iter().map(|s| self.visit(s)),
            RcDoc::hardline(),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::pretty_print::PrettyRenderer;

    #[test]
    fn placeholder() {
        let _ = PrettyRenderer;
    }
}
