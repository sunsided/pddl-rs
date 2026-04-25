use std::ops::Deref;

use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::{ActionSymbol, DurativeActionSymbol, Predicate, PreferenceName};
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;

impl sealed::Sealed for Predicate {}
impl sealed::Sealed for ActionSymbol {}
impl sealed::Sealed for DurativeActionSymbol {}
impl sealed::Sealed for PreferenceName {}

impl Visitor<Predicate, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &Predicate) -> RcDoc<'static> {
        value.deref().accept(self)
    }
}

impl Visitor<ActionSymbol, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &ActionSymbol) -> RcDoc<'static> {
        value.deref().accept(self)
    }
}

impl Visitor<DurativeActionSymbol, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &DurativeActionSymbol) -> RcDoc<'static> {
        value.deref().accept(self)
    }
}

impl Visitor<PreferenceName, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &PreferenceName) -> RcDoc<'static> {
        value.deref().accept(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;

    #[test]
    fn predicate_works() {
        let x = Predicate::new_string("at");
        assert_eq!(prettify!(x, 10), "at");
    }

    #[test]
    fn action_symbol_works() {
        let x = ActionSymbol::new_string("mov-B");
        assert_eq!(prettify!(x, 10), "mov-B");
    }
}
