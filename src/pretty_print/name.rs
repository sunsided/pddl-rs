use crate::pretty_print::PrettyRenderer;
use crate::types::{FunctionSymbol, Name, Variable};
use crate::visitor::Visitor;
use pretty::RcDoc;

impl<'a> Visitor<Name<'a>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Name<'a>) -> RcDoc<'a> {
        RcDoc::text(value.as_str())
    }
}

impl<'a> Visitor<Variable<'a>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &Variable<'a>) -> RcDoc<'a> {
        RcDoc::text("?").append(value.as_str())
    }
}

impl<'a> Visitor<FunctionSymbol<'a>, RcDoc<'a>> for PrettyRenderer {
    fn visit(&self, value: &FunctionSymbol<'a>) -> RcDoc<'a> {
        RcDoc::text(value.as_str())
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
        let x = Variable::new("var");
        assert_eq!(prettify!(x, 10), "?var");
    }

    #[test]
    fn function_symbol_works() {
        let x = FunctionSymbol::new("fun-sym");
        assert_eq!(prettify!(x, 10), "fun-sym");
    }
}
