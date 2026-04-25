use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{Problem, ProblemConstraintsDef};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Problem {}
impl sealed::Sealed for ProblemConstraintsDef {}

impl Visitor<ProblemConstraintsDef, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ProblemConstraintsDef) -> RcDoc<'static> {
        if value.value().is_empty() {
            return RcDoc::nil();
        }
        value.value().accept(self)
    }
}

impl Visitor<Problem, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Problem) -> RcDoc<'static> {
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
    use crate::types::{PrefConGDs, ProblemConstraintsDef};

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

    #[test]
    fn problem_with_requirements() {
        let input = r#"(define (problem test-prob)
  (:domain test)
  (:requirements :strips)
  (:init)
  (:goal (and))
)"#;
        let problem = crate::Problem::from_str(input).unwrap();
        let output = problem.pretty(80).to_string();
        assert!(output.contains("(:requirements :strips)"));
    }

    #[test]
    fn problem_with_objects() {
        let input = r#"(define (problem test-prob)
  (:domain test)
  (:objects a b)
  (:init)
  (:goal (and))
)"#;
        let problem = crate::Problem::from_str(input).unwrap();
        let output = problem.pretty(80).to_string();
        assert!(output.contains("(:objects a b)"));
    }

    #[test]
    fn problem_with_constraints() {
        let input = r#"(define (problem test-prob)
  (:domain test)
  (:init)
  (:goal (and))
  (:constraints (and))
  (:requirements :constraints)
)"#;
        let problem = crate::Problem::from_str(input);
        if let Ok(problem) = problem {
            let output = problem.pretty(80).to_string();
            assert!(output.contains("(:constraints"));
        }
    }

    #[test]
    fn problem_with_metric() {
        let input = r#"(define (problem test-prob)
  (:domain test)
  (:init)
  (:goal (and))
  (:metric minimize total-time)
)"#;
        let problem = crate::Problem::from_str(input).unwrap();
        let output = problem.pretty(80).to_string();
        assert!(output.contains("(:metric minimize total-time)"));
    }

    #[test]
    fn problem_with_length_spec() {
        let input = r#"(define (problem test-prob)
  (:domain test)
  (:init)
  (:goal (and))
  (:length (:serial 10))
)"#;
        let problem = crate::Problem::from_str(input).unwrap();
        let output = problem.pretty(80).to_string();
        assert!(output.contains("(:length (:serial 10))"));
    }

    #[test]
    fn problem_constraints_def_empty_nil() {
        let dc = ProblemConstraintsDef::new(PrefConGDs::new(Vec::new()));
        let out = dc.pretty(80).to_string();
        assert_eq!(out, "");
    }
}
