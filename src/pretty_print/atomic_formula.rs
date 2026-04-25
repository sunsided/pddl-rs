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

impl Visitor<AtomicFormula<Term>, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &AtomicFormula<Term>) -> RcDoc<'static> {
        match value {
            AtomicFormula::Equality(eq) => eq.accept(self),
            AtomicFormula::Predicate(pred) => pred.accept(self),
        }
    }
}

impl Visitor<EqualityAtomicFormula<Term>, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &EqualityAtomicFormula<Term>) -> RcDoc<'static> {
        RcDoc::text("(= ")
            .append(value.first().accept(self))
            .append(" ")
            .append(value.second().accept(self))
            .append(")")
    }
}

impl Visitor<PredicateAtomicFormula<Term>, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PredicateAtomicFormula<Term>) -> RcDoc<'static> {
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

impl Visitor<Literal<Term>, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Literal<Term>) -> RcDoc<'static> {
        match value {
            Literal::AtomicFormula(af) => af.accept(self),
            Literal::NotAtomicFormula(af) => {
                RcDoc::text("(not ").append(af.accept(self)).append(")")
            }
        }
    }
}

impl Visitor<Literal<Name>, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Literal<Name>) -> RcDoc<'static> {
        match value {
            Literal::AtomicFormula(af) => af.accept(self),
            Literal::NotAtomicFormula(af) => {
                RcDoc::text("(not ").append(af.accept(self)).append(")")
            }
        }
    }
}

impl Visitor<AtomicFormula<Name>, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &AtomicFormula<Name>) -> RcDoc<'static> {
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
        let x = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![
                Term::new_name(Name::new("B")),
                Term::new_name(Name::new("home")),
            ],
        );
        assert_eq!(prettify!(x, 20), "(at B home)");
    }

    #[test]
    fn literal_not_works() {
        let af = AtomicFormula::predicate(
            Predicate::string("at"),
            vec![Term::new_name(Name::new("B"))],
        );
        let x = Literal::<Term>::new_not(af);
        assert_eq!(prettify!(x, 20), "(not (at B))");
    }

    #[test]
    fn literal_name_atomic_works() {
        use crate::AtomicFormula as NameAtomicFormula;
        let af = NameAtomicFormula::predicate(
            Predicate::string("p"),
            vec![Name::new("x"), Name::new("y")],
        );
        let lit = Literal::<Name>::new(af);
        assert_eq!(prettify!(lit, 20), "(p x y)");
    }

    #[test]
    fn literal_name_not_atomic_works() {
        use crate::AtomicFormula as NameAtomicFormula;
        let af = NameAtomicFormula::predicate(Predicate::string("p"), vec![Name::new("x")]);
        let lit = Literal::<Name>::new_not(af);
        assert_eq!(prettify!(lit, 20), "(not (p x))");
    }

    #[test]
    fn atomic_formula_name_equality_works() {
        let eq = AtomicFormula::<Name>::new_equality(Name::new("x"), Name::new("y"));
        assert_eq!(prettify!(eq, 20), "(= x y)");
    }

    #[test]
    fn atomic_formula_name_predicate_empty_works() {
        let pred = AtomicFormula::<Name>::new_predicate(Predicate::string("p"), Vec::<Name>::new());
        assert_eq!(prettify!(pred, 20), "(p)");
    }

    #[test]
    fn atomic_formula_name_predicate_non_empty_works() {
        let pred = AtomicFormula::<Name>::new_predicate(
            Predicate::string("p"),
            vec![Name::new("x"), Name::new("y")],
        );
        assert_eq!(prettify!(pred, 20), "(p x y)");
    }

    #[test]
    fn atomic_formula_term_empty_args_works() {
        let pred = AtomicFormula::<Term>::new_predicate(Predicate::string("p"), Vec::<Term>::new());
        assert_eq!(prettify!(pred, 20), "(p)");
    }
}
