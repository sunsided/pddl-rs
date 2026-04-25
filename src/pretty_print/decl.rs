use std::ops::Deref;

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    Constants, Functions, InitElement, InitElements, LengthSpec, MetricSpec, Objects,
    PredicateDefinitions, ProblemGoalDefinition, Requirements, Types,
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
impl sealed::Sealed for ProblemGoalDefinition {}
impl sealed::Sealed for MetricSpec {}
impl sealed::Sealed for LengthSpec {}
impl sealed::Sealed for InitElement {}

impl Visitor<Requirements, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Requirements) -> RcDoc<'static> {
        RcDoc::intersperse(value.iter().map(|r| r.accept(self)), RcDoc::softline())
    }
}

impl Visitor<Types, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Types) -> RcDoc<'static> {
        value.values().accept(self)
    }
}

impl Visitor<Constants, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Constants) -> RcDoc<'static> {
        value.deref().accept(self)
    }
}

impl Visitor<PredicateDefinitions, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PredicateDefinitions) -> RcDoc<'static> {
        RcDoc::intersperse(
            value.values().iter().map(|p| p.accept(self)),
            RcDoc::softline(),
        )
    }
}

impl Visitor<Functions, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Functions) -> RcDoc<'static> {
        value.values().accept(self)
    }
}

impl Visitor<Objects, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Objects) -> RcDoc<'static> {
        value.values().accept(self)
    }
}

impl Visitor<InitElements, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &InitElements) -> RcDoc<'static> {
        RcDoc::intersperse(
            value.values().iter().map(|ie| ie.accept(self)),
            RcDoc::softline(),
        )
    }
}

impl Visitor<InitElement, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &InitElement) -> RcDoc<'static> {
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

impl Visitor<ProblemGoalDefinition, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ProblemGoalDefinition) -> RcDoc<'static> {
        value.deref().accept(self)
    }
}

impl Visitor<MetricSpec, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &MetricSpec) -> RcDoc<'static> {
        value
            .optimization()
            .accept(self)
            .append(RcDoc::softline())
            .append(value.expression().accept(self))
    }
}

impl Visitor<LengthSpec, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &LengthSpec) -> RcDoc<'static> {
        let mut doc = RcDoc::text("(:length");
        if let Some(s) = value.serial() {
            doc = doc.append(RcDoc::text(format!(" (:serial {s})")));
        }
        if let Some(p) = value.parallel() {
            doc = doc.append(RcDoc::text(format!(" (:parallel {p})")));
        }
        doc.append(")")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{Literal, PreconditionGoalDefinitions, ToTyped};

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
            crate::MetricFluentExpression::total_time(),
        );
        assert_eq!(prettify!(ms, 30), "minimize total-time");
    }

    #[test]
    fn length_spec_works() {
        let ls = crate::LengthSpec::new_serial(10);
        assert_eq!(prettify!(ls, 30), "(:length (:serial 10))");
    }

    #[test]
    fn types_works() {
        use crate::TypedNames;
        let t = Types::new(TypedNames::from_iter([
            crate::Name::new("block").to_typed(crate::Type::OBJECT)
        ]));
        assert_eq!(prettify!(t, 20), "block");
    }

    #[test]
    fn constants_works() {
        use crate::TypedNames;
        let c = Constants::new(TypedNames::from_iter([
            crate::Name::new("c1").to_typed(crate::Type::OBJECT)
        ]));
        assert_eq!(prettify!(c, 20), "c1");
    }

    #[test]
    fn functions_works() {
        use crate::{AtomicFunctionSkeleton, FunctionSymbol, FunctionType, FunctionTyped};
        let ft = FunctionTyped::new(
            AtomicFunctionSkeleton::new(FunctionSymbol::from("fuel"), vec![].into()),
            FunctionType::NUMBER,
        );
        let f = Functions::from_iter([ft]);
        // FunctionTypedList renders AtomicFunctionSkeleton as (symbol)
        assert_eq!(prettify!(f, 20), "(fuel)");
    }

    #[test]
    fn objects_works() {
        let o = Objects::new([crate::Name::new("o1").to_typed(crate::Type::OBJECT)]);
        assert_eq!(prettify!(o, 20), "o1");
    }

    #[test]
    fn init_elements_multi_works() {
        let lit1 = crate::AtomicFormula::predicate(
            crate::Predicate::string("block"),
            vec![crate::Name::new("a")],
        );
        let lit2 = crate::AtomicFormula::predicate(
            crate::Predicate::string("block"),
            vec![crate::Name::new("b")],
        );
        let ie = InitElements::new(vec![
            InitElement::Literal(Literal::new(lit1)),
            InitElement::Literal(Literal::new(lit2)),
        ]);
        let out = prettify!(ie, 40);
        assert!(out.contains("(block a)"));
        assert!(out.contains("(block b)"));
    }

    #[test]
    fn init_element_literal_works() {
        let af = crate::AtomicFormula::predicate(
            crate::Predicate::string("block"),
            vec![crate::Name::new("a")],
        );
        let ie = InitElement::Literal(Literal::new(af));
        assert_eq!(prettify!(ie, 20), "(block a)");
    }

    #[test]
    fn init_element_at_works() {
        let af = crate::AtomicFormula::predicate(
            crate::Predicate::string("block"),
            vec![crate::Name::new("a")],
        );
        let ie = InitElement::At(crate::Number::from(10), Literal::new(af));
        assert_eq!(prettify!(ie, 30), "(at 10 (block a))");
    }

    #[test]
    fn init_element_is_value_works() {
        use crate::{FunctionSymbol, Name, Number};
        let term = crate::BasicFunctionTerm::new(FunctionSymbol::from("x"), Vec::<Name>::new());
        let ie = InitElement::IsValue(term, Number::from(42));
        assert_eq!(prettify!(ie, 20), "(= x 42)");
    }

    #[test]
    fn init_element_is_object_works() {
        use crate::{FunctionSymbol, Name};
        let term = crate::BasicFunctionTerm::new(FunctionSymbol::from("x"), Vec::<Name>::new());
        let ie = InitElement::IsObject(term, Name::new("obj1"));
        assert_eq!(prettify!(ie, 20), "(= x obj1)");
    }

    #[test]
    fn goal_def_works() {
        let gd = crate::GoalDefinition::and(Vec::<crate::GoalDefinition>::new());
        let pre_gd = crate::PreconditionGoalDefinition::preference(
            crate::PreferenceGoalDefinition::from_gd(gd),
        );
        let gdef = ProblemGoalDefinition::new(PreconditionGoalDefinitions::new(vec![pre_gd]));
        assert_eq!(prettify!(gdef, 20), "(and)");
    }

    #[test]
    fn length_spec_both_works() {
        let ls = crate::LengthSpec::new(Some(10), Some(5));
        assert_eq!(prettify!(ls, 40), "(:length (:serial 10) (:parallel 5))");
    }

    #[test]
    fn length_spec_parallel_works() {
        let ls = crate::LengthSpec::new_parallel(5);
        assert_eq!(prettify!(ls, 30), "(:length (:parallel 5))");
    }

    #[test]
    fn predicate_definitions_works() {
        use crate::AtomicFormulaSkeleton;
        let preds = PredicateDefinitions::new(vec![AtomicFormulaSkeleton::new(
            crate::Predicate::string("at"),
            vec![].into(),
        )]);
        assert_eq!(prettify!(preds, 20), "(at)");
    }
}
