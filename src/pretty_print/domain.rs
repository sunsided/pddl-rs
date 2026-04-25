use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{Domain, DomainConstraintsDef};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Domain {}
impl sealed::Sealed for DomainConstraintsDef {}

impl<'a> Visitor<DomainConstraintsDef, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &DomainConstraintsDef) -> RcDoc<'a> {
        if value.value().is_empty() {
            return RcDoc::nil();
        }
        value.value().accept(self)
    }
}

impl<'a> Visitor<Domain, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Domain) -> RcDoc<'a> {
        let mut doc = RcDoc::text("(")
            .append(RcDoc::text("define"))
            .append(RcDoc::hardline())
            .append(RcDoc::text("(domain "))
            .append(value.name().accept(self))
            .append(")");

        if !value.extends().is_empty() {
            doc = doc
                .append(RcDoc::hardline())
                .append(RcDoc::text("(:extends"))
                .append(RcDoc::softline())
                .append(RcDoc::intersperse(
                    value.extends().iter().map(|n| n.accept(self)),
                    RcDoc::softline(),
                ))
                .nest(4)
                .group()
                .append(")");
        }

        if !value.requirements().is_empty() {
            doc = doc.append(RcDoc::hardline()).append(self.section(
                "requirements",
                value.requirements().iter().map(|r| r.accept(self)),
            ));
        }

        if !value.types().is_empty() {
            doc = doc
                .append(RcDoc::hardline())
                .append(RcDoc::text("(:types"))
                .append(RcDoc::softline())
                .append(value.types().accept(self))
                .nest(4)
                .group()
                .append(")");
        }

        if !value.constants().is_empty() {
            doc = doc
                .append(RcDoc::hardline())
                .append(RcDoc::text("(:constants"))
                .append(RcDoc::softline())
                .append(value.constants().accept(self))
                .nest(4)
                .group()
                .append(")");
        }

        if !value.predicates().is_empty() {
            doc = doc.append(RcDoc::hardline()).append(self.section(
                "predicates",
                value.predicates().iter().map(|p| p.accept(self)),
            ));
        }

        if !value.functions().is_empty() {
            doc = doc.append(RcDoc::hardline()).append(self.section(
                "functions",
                value.functions().values().iter().map(|f| f.accept(self)),
            ));
        }

        if !value.constraints().is_empty() {
            doc = doc
                .append(RcDoc::hardline())
                .append(self.section("constraints", [value.constraints().accept(self)]));
        }

        doc.append(RcDoc::hardline()).append(")").nest(2).group()
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::Parser;
    use crate::pretty_print::Pretty;

    #[test]
    fn domain_basic() {
        let input = r#"(define (domain test)
  (:requirements :strips)
  (:types block)
  (:predicates (on ?x ?y - block))
)"#;
        let domain = crate::Domain::from_str(input).unwrap();
        let output = domain.pretty(80).to_string();
        assert!(output.contains("(define"));
        assert!(output.contains("(domain test)"));
    }
}
