use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{
    AtomicFormula, EqualityAtomicFormula, Literal, Name, PredicateAtomicFormula, Term,
};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for AtomicFormula<Term> {}
impl sealed::Sealed for EqualityAtomicFormula<Term> {}
impl sealed::Sealed for PredicateAtomicFormula<Term> {}
impl sealed::Sealed for Literal<Name> {}
impl sealed::Sealed for Literal<Term> {}

impl<'a> Visitor<AtomicFormula<Term>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &AtomicFormula<Term>) -> RcDoc<'a> {
        match value {
            AtomicFormula::Equality(eq) => eq.accept(self),
            AtomicFormula::Predicate(pred) => pred.accept(self),
        }
    }
}

impl<'a> Visitor<EqualityAtomicFormula<Term>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &EqualityAtomicFormula<Term>) -> RcDoc<'a> {
        RcDoc::text("(= ")
            .append(value.first().accept(self))
            .append(" ")
            .append(value.second().accept(self))
            .append(")")
    }
}

impl<'a> Visitor<PredicateAtomicFormula<Term>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &PredicateAtomicFormula<Term>) -> RcDoc<'a> {
        if value.values().is_empty() {
            return RcDoc::text("(")
                .append(value.predicate().accept(self))
                .append(")");
        }
        RcDoc::text("(")
            .append(value.predicate().accept(self))
            .append(RcDoc::softline())
            .append(RcDoc::intersperse(
                value.values().iter().map(|t| t.accept(self)),
                RcDoc::softline(),
            ))
            .nest(4)
            .group()
            .append(")")
    }
}

impl<'a> Visitor<Literal<Term>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Literal<Term>) -> RcDoc<'a> {
        match value {
            Literal::AtomicFormula(af) => af.accept(self),
            Literal::NotAtomicFormula(af) => {
                RcDoc::text("(not ").append(af.accept(self)).append(")")
            }
        }
    }
}

impl<'a> Visitor<Literal<Name>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Literal<Name>) -> RcDoc<'a> {
        match value {
            Literal::AtomicFormula(af) => af.accept(self),
            Literal::NotAtomicFormula(af) => {
                RcDoc::text("(not ").append(af.accept(self)).append(")")
            }
        }
    }
}

impl<'a> Visitor<AtomicFormula<Name>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &AtomicFormula<Name>) -> RcDoc<'a> {
        match value {
            AtomicFormula::Equality(eq) => RcDoc::text("(= ")
                .append(eq.first().accept(self))
                .append(" ")
                .append(eq.second().accept(self))
                .append(")"),
            AtomicFormula::Predicate(pred) => {
                if pred.values().is_empty() {
                    return RcDoc::text("(")
                        .append(pred.predicate().accept(self))
                        .append(")");
                }
                RcDoc::text("(")
                    .append(pred.predicate().accept(self))
                    .append(RcDoc::softline())
                    .append(RcDoc::intersperse(
                        pred.values().iter().map(|n| n.accept(self)),
                        RcDoc::softline(),
                    ))
                    .nest(4)
                    .group()
                    .append(")")
            }
        }
    }
}

impl sealed::Sealed for AtomicFormula<Name> {}
impl sealed::Sealed for EqualityAtomicFormula<Name> {}
impl sealed::Sealed for PredicateAtomicFormula<Name> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::{Name, Predicate};

    #[test]
    fn equality_works() {
        let x = AtomicFormula::<Term>::new_equality(
            Term::new_name(Name::new("x")),
            Term::new_name(Name::new("y")),
        );
        assert_eq!(prettify!(x, 20), "(= x y)");
    }

    #[test]
    fn predicate_formula_works() {
        let x = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![
                Term::new_name(Name::new("B")),
                Term::new_name(Name::new("home")),
            ],
        );
        assert_eq!(prettify!(x, 20), "(at B home)");
    }

    #[test]
    fn literal_not_works() {
        let af = AtomicFormula::new_predicate(
            Predicate::new_string("at"),
            vec![Term::new_name(Name::new("B"))],
        );
        let x = Literal::<Term>::new_not(af);
        assert_eq!(prettify!(x, 20), "(not (at B))");
    }
}
