use crate::pretty_print::{sealed, PrettyRenderer};
use crate::types::FunctionType;
use crate::visitor::{Accept, Visitor};
use pretty::RcDoc;
use std::ops::Deref;

impl sealed::Sealed for FunctionType {}

impl Visitor<FunctionType, RcDoc<'static>> for PrettyRenderer {
    fn visit(&self, value: &FunctionType) -> RcDoc<'static> {
        let t: &crate::types::Type = value.deref();
        t.accept(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pretty_print::prettify;
    use crate::visitor::Accept;
    use crate::FunctionType;

    #[test]
    fn function_type_number() {
        let x = FunctionType::NUMBER;
        assert_eq!(prettify!(x, 10), "number");
    }

    #[test]
    fn function_type_custom() {
        let x = FunctionType::new("fuel");
        assert_eq!(prettify!(x, 10), "fuel");
    }
}
