use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{FunctionSymbol, Name, Variable};
use crate::visitor::Visitor;
use pretty::RcDoc;

impl sealed::Sealed for Name {}
impl sealed::Sealed for Variable {}
impl sealed::Sealed for FunctionSymbol {}

impl Visitor<Name, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Name) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<Variable, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Variable) -> RcDoc<'static> {
        RcDoc::text(value.to_string())
    }
}

impl Visitor<FunctionSymbol, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &FunctionSymbol) -> RcDoc<'static> {
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
