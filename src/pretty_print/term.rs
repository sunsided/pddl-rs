use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{BasicFunctionTerm, FunctionTerm, Term};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Term {}
impl sealed::Sealed for FunctionTerm {}
impl sealed::Sealed for BasicFunctionTerm {}

impl<'a> Visitor<Term, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Term) -> RcDoc<'a> {
        match value {
            Term::Name(n) => n.accept(self),
            Term::Variable(v) => v.accept(self),
            Term::Function(ft) => ft.accept(self),
        }
    }
}

impl<'a> Visitor<FunctionTerm, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FunctionTerm) -> RcDoc<'a> {
        if value.terms().is_empty() {
            return value.symbol().accept(self);
        }
        RcDoc::text("(")
            .append(value.symbol().accept(self))
            .append(RcDoc::softline())
            .append(RcDoc::intersperse(
                value.terms().iter().map(|t| t.accept(self)),
                RcDoc::softline(),
            ))
            .nest(4)
            .group()
            .append(")")
    }
}

impl<'a> Visitor<BasicFunctionTerm, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &BasicFunctionTerm) -> RcDoc<'a> {
        if value.names().is_empty() {
            return value.symbol().accept(self);
        }
        RcDoc::text("(")
            .append(value.symbol().accept(self))
            .append(RcDoc::softline())
            .append(RcDoc::intersperse(
                value.names().iter().map(|n| n.accept(self)),
                RcDoc::softline(),
            ))
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
    use crate::{FunctionSymbol, Name, Variable};

    #[test]
    fn term_name_works() {
        let x = Term::new_name(Name::new("foo"));
        assert_eq!(prettify!(x, 10), "foo");
    }

    #[test]
    fn term_variable_works() {
        let x = Term::new_variable(Variable::from("x"));
        assert_eq!(prettify!(x, 10), "?x");
    }

    #[test]
    fn function_term_works() {
        let ft = FunctionTerm::new(
            FunctionSymbol::from("f"),
            vec![
                Term::new_name(Name::new("a")),
                Term::new_variable(Variable::from("x")),
            ],
        );
        assert_eq!(prettify!(ft, 20), "(f a ?x)");
    }
}
