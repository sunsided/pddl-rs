use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{AtomicFormulaSkeleton, AtomicFunctionSkeleton};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for AtomicFormulaSkeleton {}
impl sealed::Sealed for AtomicFunctionSkeleton {}

impl<'a> Visitor<AtomicFormulaSkeleton, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &AtomicFormulaSkeleton) -> RcDoc<'a> {
        if value.variables().is_empty() {
            return RcDoc::text("(")
                .append(value.predicate().accept(self))
                .append(")");
        }
        RcDoc::text("(")
            .append(value.predicate().accept(self))
            .append(RcDoc::softline())
            .append(value.variables().accept(self))
            .nest(4)
            .group()
            .append(")")
    }
}

impl<'a> Visitor<AtomicFunctionSkeleton, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &AtomicFunctionSkeleton) -> RcDoc<'a> {
        if value.variables().is_empty() {
            return value.symbol().accept(self);
        }
        RcDoc::text("(")
            .append(value.symbol().accept(self))
            .append(RcDoc::softline())
            .append(value.variables().accept(self))
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
    use crate::{AtomicFormulaSkeleton, FunctionSymbol, Predicate, ToTyped, Type, Variable};

    #[test]
    fn atomic_formula_skeleton_no_vars() {
        let x = AtomicFormulaSkeleton::new(Predicate::new_string("block"), vec![].into());
        assert_eq!(prettify!(x, 20), "(block)");
    }

    #[test]
    fn atomic_formula_skeleton_with_vars() {
        let x = AtomicFormulaSkeleton::new(
            Predicate::new_string("at"),
            vec![
                Variable::new_string("x").to_typed(Type::new_exactly("physob")),
                Variable::new_string("y").to_typed(Type::new_exactly("location")),
            ]
            .into(),
        );
        assert_eq!(prettify!(x, 40), "(at ?x - physob ?y - location)");
    }

    #[test]
    fn atomic_function_skeleton() {
        let x = AtomicFunctionSkeleton::new(
            FunctionSymbol::from("battery-level"),
            vec![Variable::new_string("r").to_typed(Type::new_exactly("rover"))].into(),
        );
        assert_eq!(prettify!(x, 40), "(battery-level ?r - rover)");
    }
}
