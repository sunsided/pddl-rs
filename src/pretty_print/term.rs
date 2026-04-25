use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{BasicFunctionTerm, FunctionTerm, Term};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Term {}
impl sealed::Sealed for FunctionTerm {}
impl sealed::Sealed for BasicFunctionTerm {}

impl Visitor<Term, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Term) -> RcDoc<'static> {
        match value {
            Term::Name(n) => n.accept(self),
            Term::Variable(v) => v.accept(self),
            Term::Function(ft) => ft.accept(self),
        }
    }
}

impl Visitor<FunctionTerm, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &FunctionTerm) -> RcDoc<'static> {
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

impl Visitor<BasicFunctionTerm, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &BasicFunctionTerm) -> RcDoc<'static> {
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
    use crate::{BasicFunctionTerm, FunctionSymbol, FunctionTerm, Name, Variable};

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

    #[test]
    fn term_function_works() {
        let ft = FunctionTerm::new(
            FunctionSymbol::from("g"),
            vec![Term::new_name(Name::new("x"))],
        );
        let t = Term::new_function(ft);
        assert_eq!(prettify!(t, 20), "(g x)");
    }

    #[test]
    fn function_term_empty_terms_works() {
        let ft = FunctionTerm::new(FunctionSymbol::from("f"), Vec::<Term>::new());
        assert_eq!(prettify!(ft, 20), "f");
    }

    #[test]
    fn basic_function_term_empty_works() {
        let bft = BasicFunctionTerm::new(FunctionSymbol::from("x"), Vec::<Name>::new());
        assert_eq!(prettify!(bft, 20), "x");
    }

    #[test]
    fn basic_function_term_non_empty_works() {
        let bft = BasicFunctionTerm::new(
            FunctionSymbol::from("x"),
            vec![Name::new("a"), Name::new("b")],
        );
        assert_eq!(prettify!(bft, 20), "(x a b)");
    }
}
