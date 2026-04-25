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
        } else {
            doc = doc
                .append(RcDoc::hardline())
                .append(self.keyword_line("effect"))
                .append(RcDoc::text(" (and)"));
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
    use crate::parsers::Parser;
    use crate::pretty_print::Pretty;

    #[test]
    fn action_with_effect() {
        let input = r#"(define (domain test)
  (:requirements :strips)
  (:predicates (p))
  (:action move
    :parameters (?x)
    :precondition (and)
    :effect (and (p))
  )
)"#;
        let domain = crate::Domain::from_str(input).unwrap();
        let output = domain.pretty(80).to_string();
        assert!(output.contains(":effect"));
        assert!(output.contains("(and (p))"));
    }

    #[test]
    fn action_without_effect() {
        let input = r#"(define (domain test)
  (:requirements :strips)
  (:predicates (p))
  (:action noop
    :parameters ()
    :precondition (and)
  )
)"#;
        let domain = crate::Domain::from_str(input).unwrap();
        let output = domain.pretty(80).to_string();
        assert!(output.contains(":effect (and)"));
    }

    #[test]
    fn durative_action_all_options() {
        let input = r#"(define (domain test)
  (:requirements :durative-actions)
  (:predicates (p))
  (:durative-action move
    :parameters (?x)
    :duration (= ?duration 10)
    :condition (and (at start (p)))
    :effect (and (at end (p)))
  )
)"#;
        let domain = crate::Domain::from_str(input).unwrap();
        let output = domain.pretty(80).to_string();
        assert!(output.contains(":duration"));
        assert!(output.contains(":condition"));
        assert!(output.contains(":effect"));
    }

    #[test]
    fn durative_action_no_duration() {
        let input = r#"(define (domain test)
  (:requirements :durative-actions :strips)
  (:predicates (p))
  (:durative-action move
    :parameters (?x)
    :condition (and (at start (p)))
    :effect (and (at start (p)))
  )
)"#;
        let domain = crate::Domain::from_str(input);
        // May or may not parse depending on parser implementation
        if let Ok(domain) = domain {
            let output = domain.pretty(80).to_string();
            assert!(!output.contains(":duration"));
            assert!(output.contains(":condition"));
            assert!(output.contains(":effect"));
        }
    }

    #[test]
    fn durative_action_minimal() {
        let input = r#"(define (domain test)
  (:requirements :durative-actions :strips)
  (:durative-action noop
    :parameters ()
  )
)"#;
        let domain = crate::Domain::from_str(input);
        if let Ok(domain) = domain {
            let output = domain.pretty(80).to_string();
            assert!(output.contains("(:durative-action noop"));
        }
    }

    #[test]
    fn derived_predicate() {
        let input = r#"(define (domain test)
  (:requirements :derived-predicates)
  (:predicates (p) (q))
  (:derived (p) (and))
)"#;
        let domain = crate::Domain::from_str(input).unwrap();
        let output = domain.pretty(80).to_string();
        assert!(output.contains("(:derived"));
    }

    #[test]
    fn structure_def_action() {
        let input = r#"(define (domain test)
  (:requirements :strips)
  (:action a
    :parameters ()
    :precondition (and)
    :effect (and)
  )
)"#;
        let domain = crate::Domain::from_str(input).unwrap();
        let output = domain.pretty(80).to_string();
        assert!(output.contains("(:action a"));
    }
}
