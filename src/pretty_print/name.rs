use crate::pretty_print::PrettyRenderer;
use crate::types::{FunctionSymbol, Name, Variable};
use crate::visitor::Visitor;
use pretty::RcDoc;

impl<'a> Visitor<Name, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Name) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<Variable, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Variable) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

impl<'a> Visitor<FunctionSymbol, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FunctionSymbol) -> RcDoc<'a> {
        RcDoc::text(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn name_works() {
        let x = Name::new("name");
        assert_eq!(prettify!(x, 10), "name");
    }

    #[test]
    fn variable_works() {
        let x = Variable::from("var");
        assert_eq!(prettify!(x, 10), "?var");
    }

    #[test]
    fn function_symbol_works() {
        let x = FunctionSymbol::from("fun-sym");
        assert_eq!(prettify!(x, 10), "fun-sym");
    }
}
