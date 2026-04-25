use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{Problem, ProblemConstraintsDef};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Problem {}
impl sealed::Sealed for ProblemConstraintsDef {}

impl<'a> Visitor<ProblemConstraintsDef, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &ProblemConstraintsDef) -> RcDoc<'a> {
        if value.value().is_empty() {
            return RcDoc::nil();
        }
        value.value().accept(self)
    }
}

impl<'a> Visitor<Problem, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Problem) -> RcDoc<'a> {
        let mut doc = RcDoc::text("(")
            .append(RcDoc::text("define"))
            .append(RcDoc::hardline())
            .append(RcDoc::text("(problem "))
            .append(value.name().accept(self))
            .append(")")
            .append(RcDoc::hardline())
            .append(RcDoc::text("(:domain "))
            .append(value.domain().accept(self))
            .append(")");

        if !value.requirements().is_empty() {
            doc = doc.append(RcDoc::hardline()).append(self.section(
                "requirements",
                value.requirements().iter().map(|r| r.accept(self)),
            ));
        }

        if !value.objects().is_empty() {
            doc = doc
                .append(RcDoc::hardline())
                .append(RcDoc::text("(:objects"))
                .append(RcDoc::softline())
                .append(value.objects().accept(self))
                .nest(4)
                .group()
                .append(")");
        }

        doc = doc
            .append(RcDoc::hardline())
            .append(self.section("init", value.init().iter().map(|ie| ie.accept(self))));

        doc = doc
            .append(RcDoc::hardline())
            .append(self.section("goal", [value.goals().accept(self)]));

        if !value.constraints().is_empty() {
            doc = doc.append(RcDoc::hardline()).append(self.section(
                "constraints",
                value.constraints().iter().map(|c| c.accept(self)),
            ));
        }

        if let Some(metric) = value.metric_spec() {
            doc = doc
                .append(RcDoc::hardline())
                .append(self.section("metric", [metric.accept(self)]));
        }

        if let Some(length) = value.length_spec() {
            doc = doc.append(RcDoc::hardline()).append(length.accept(self));
        }

        doc.append(RcDoc::hardline()).append(")").nest(2).group()
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::Parser;
    use crate::pretty_print::Pretty;

    #[test]
    fn problem_basic() {
        let input = r#"(define (problem test-prob)
  (:domain test)
  (:init (block a) (block b))
  (:goal (and))
)"#;
        let problem = crate::Problem::from_str(input).unwrap();
        let output = problem.pretty(80).to_string();
        assert!(output.contains("(define"));
        assert!(output.contains("(problem test-prob)"));
        assert!(output.contains("(:domain test)"));
    }
}
